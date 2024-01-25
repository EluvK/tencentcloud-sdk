use async_trait::async_trait;

use reqwest::{
    header::{self, HeaderMap, HeaderName, AUTHORIZATION},
    Request, Response,
};
use reqwest_middleware::{Middleware, Next, Result};

use ring::hmac;
use task_local_extensions::Extensions;
use time::OffsetDateTime;

use sha2::{Digest, Sha256};

use crate::client::constant::ACTION_HEADER;

pub struct SignatureContext {
    pub ak: String,
    pub sk: String,
    pub signed_headers: Option<Vec<HeaderName>>,
    pub service_name: String,
}

pub struct SignatureMiddleware;

#[async_trait]
impl Middleware for SignatureMiddleware {
    async fn handle(
        &self,
        mut req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        if let Some(context) = extensions.get::<SignatureContext>() {
            add_authorization(&mut req, context)?;
        }
        next.run(req, extensions).await
    }
}

const ALGORITHM: &str = "TC3-HMAC-SHA256";
const TIME_HEADER: &str = "X-TC-Timestamp";
const TC_VERSION: &str = "X-TC-Version";

fn hmac_sha256(key: &[u8], data: &[u8]) -> hmac::Tag {
    let key = hmac::Key::new(hmac::HMAC_SHA256, key);
    hmac::sign(&key, data)
}

pub fn add_authorization(req: &mut Request, context: &SignatureContext) -> anyhow::Result<String> {
    let date = OffsetDateTime::now_utc().date();
    let ts = OffsetDateTime::now_utc().unix_timestamp();
    req.headers_mut().insert(TIME_HEADER, ts.into());
    req.headers_mut()
        .insert(TC_VERSION, "2017-03-12".parse().unwrap());

    let host = req.url().host_str().map(|s| s.to_owned());
    if let Some(host) = host {
        req.headers_mut()
            .insert(header::HOST, host.parse().unwrap());
    }
    if req.headers().get(header::CONTENT_TYPE).is_none() {
        req.headers_mut()
            .insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
    }

    let scope = format!("{}/{}/tc3_request", date, context.service_name);

    let body = {
        let mut hasher = Sha256::new();
        hasher.update(req.body().and_then(|b| b.as_bytes()).unwrap_or_default());
        let hash = hasher.finalize();
        hex::encode(hash)
    };

    let (headers, signed_headers) = get_headers(req.headers(), context.signed_headers.as_deref())?;
    let hashed_request = get_hashed_request(req, &headers, &signed_headers, &body);

    let string_to_sign = format!("{ALGORITHM}\n{ts}\n{scope}\n{hashed_request}");
    // println!("string_to_sign: \n{string_to_sign}\n\n");

    let derived_sk = hmac_sha256(
        hmac_sha256(
            hmac_sha256(
                format!("TC3{}", context.sk).as_bytes(),
                format!("{date}").as_bytes(),
            )
            .as_ref(),
            context.service_name.as_bytes(),
        )
        .as_ref(),
        "tc3_request".as_bytes(),
    );

    let signature =
        hex::encode(hmac_sha256(derived_sk.as_ref(), string_to_sign.as_bytes()).as_ref());

    let authorization = format!(
        "{ALGORITHM} Credential={}/{scope}, SignedHeaders={signed_headers}, Signature={signature}",
        context.ak
    );
    // println!("authorization:\n{authorization:?}\n");
    req.headers_mut()
        .insert(AUTHORIZATION, authorization.parse()?);

    Ok(authorization)
}

const RFC3986_RESERVED_CHARACTERS: &percent_encoding::AsciiSet =
    &percent_encoding::NON_ALPHANUMERIC
        .remove(b'_')
        .remove(b'-')
        .remove(b'~')
        .remove(b'.');

fn precent_encode_rfc3986(data: &[u8]) -> percent_encoding::PercentEncode {
    percent_encoding::percent_encode(data, RFC3986_RESERVED_CHARACTERS)
}

fn get_hashed_request(req: &Request, headers: &str, signed_headers: &str, body: &str) -> String {
    let method = req.method().as_str().to_uppercase();
    let url = "/";
    // extract query pairs with ascending order
    let mut query: Vec<_> = req.url().query_pairs().collect();

    // NOTED: According to the current implementation of APIG
    // should sort first and then precent encode key/value.
    query.sort_unstable();
    let query = query
        .into_iter()
        .map(|(key, value)| {
            format!(
                "{key}={value}",
                key = precent_encode_rfc3986(key.as_bytes()),
                value = precent_encode_rfc3986(value.as_bytes()),
            )
        })
        .collect::<Vec<_>>()
        .join("&");

    let request = format!("{method}\n{url}\n{query}\n{headers}\n{signed_headers}\n{body}");
    // println!("hashed_request: \n{request}\n\n");

    let mut hasher = Sha256::new();
    hasher.update(&request);
    let hash = hasher.finalize();
    hex::encode(hash)
}

fn get_headers(
    headers: &HeaderMap,
    signed_headers: Option<&[HeaderName]>,
) -> anyhow::Result<(String, String)> {
    let mut signed_headers = signed_headers.unwrap_or_default().to_vec();
    if headers.get(ACTION_HEADER).is_some() {
        signed_headers.push(ACTION_HEADER.parse().unwrap());
    }
    signed_headers.sort_unstable_by(|name1, name2| name1.as_str().cmp(name2.as_str()));
    signed_headers.dedup();
    let headers_string = signed_headers
        .iter()
        .map(|key| match headers.get(key).map(|value| value.to_str()) {
            Some(Ok(value)) => Ok(format!("{key}:{}\n", value.trim().to_lowercase())),
            _ => Err(anyhow::anyhow!("extracting {key:?} in headers fails")),
        })
        .collect::<anyhow::Result<Vec<_>, _>>()?
        .join("");
    Ok((headers_string, signed_headers.join(";")))
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use reqwest::{Client, Method, RequestBuilder, Url};
    use serde_json::json;

    use crate::client::constant::ACTION_HEADER;

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_sign() {
        let client = Client::new();
        let req = Request::new(
            Method::POST,
            Url::parse("https://cvm.tencentcloudapi.com").unwrap(),
        );
        let req_builder = RequestBuilder::from_parts(client, req)
            .json(&json!(
                {"Limit": 1,}
            ))
            .header("host", "cvm.tencentcloudapi.com")
            .header(ACTION_HEADER, "describeinstances");
        let mut req = req_builder.build().unwrap();

        let signature_context = SignatureContext {
            ak: "ak".into(),
            sk: "sk".into(),
            signed_headers: Some(vec![
                HeaderName::from_str("content-type").unwrap(),
                HeaderName::from_str("host").unwrap(),
                HeaderName::from_str(ACTION_HEADER).unwrap(),
            ]),
            service_name: "cvm".into(),
        };

        let r = add_authorization(&mut req, &signature_context).unwrap();

        println!("r: {r:?}");
    }
}
