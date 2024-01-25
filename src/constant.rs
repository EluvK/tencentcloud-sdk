
use serde::Serialize;
// use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Clone, EnumString, Display, Serialize)]
pub enum Region {
    // 亚太东南（曼谷）
    #[strum(serialize = "ap-bangkok")]
    #[serde(rename = "ap-bangkok")]
    Bangkok,
    // 华北地区（北京）
    #[strum(serialize = "ap-beijing")]
    Beijing,
    // 西南地区（成都）
    #[strum(serialize = "ap-chengdu")]
    Chengdu,
    // 西南地区（重庆）
    #[strum(serialize = "ap-chongqing")]
    Chongqing,
    // 华南地区（广州）
    #[strum(serialize = "ap-guangzhou")]
    Guangzhou,
    // 港澳台地区（中国香港）
    #[strum(serialize = "ap-hongkong")]
    Hongkong,
    // 亚太东南（雅加达）
    #[strum(serialize = "ap-jakarta")]
    Jakarta,
    // 亚太南部（孟买）
    #[strum(serialize = "ap-mumbai")]
    Mumbai,
    // 华东地区（南京）
    #[strum(serialize = "ap-nanjing")]
    Nanjing,
    // 亚太东北（首尔）
    #[strum(serialize = "ap-seoul")]
    Seoul,
    // 华东地区（上海）
    #[strum(serialize = "ap-shanghai")]
    Shanghai,
    // 亚太东南（新加坡）
    #[strum(serialize = "ap-singapore")]
    Singapore,
    // 亚太东北（东京）
    #[strum(serialize = "ap-tokyo")]
    Tokyo,
    // 欧洲地区（法兰克福）
    #[strum(serialize = "eu-frankfurt")]
    Frankfurt,
    // 美国东部（弗吉尼亚）
    #[strum(serialize = "na-ashburn")]
    Ashburn,
    // 美国西部（硅谷）
    #[strum(serialize = "na-siliconvalley")]
    Siliconvalley,
    // 北美地区（多伦多）
    #[strum(serialize = "na-toronto")]
    Toronto,
    // 南美地区（圣保罗）
    #[strum(serialize = "sa-saopaulo")]
    Saopaulo,
}

#[derive(Debug, Clone, EnumString, Display, Serialize)]
pub enum InstanceType {
    #[strum(serialize = "SA2.MEDIUM2")]
    SA2Medium2, // 2C2G // for test case

    #[strum(serialize = "SA2.MEDIUM8")]
    SA2Medium8, // 2C8G
    #[strum(serialize = "SA2.LARGE8")]
    SA2Large8, // 4C8G
    #[strum(serialize = "SA3.LARGE8")]
    SA3Large8, // 4C8G

    #[strum(serialize = "MA3.MEDIUM16")]
    MA3Medium16, // 2C16G
    #[strum(serialize = "M5.MEDIUM16")]
    M5Medium16, // 2C16G

    #[strum(serialize = "SA5.LARGE16")]
    SA5Large16, // 4C16G
    #[strum(serialize = "S6.LARGE16")]
    S6Large16, // 4C16G
    #[strum(serialize = "SA3.LARGE16")]
    SA3Large16, // 4C16G
    #[strum(serialize = "S5.LARGE16")]
    S5Large16, // 4C16G
    #[strum(serialize = "SA2.LARGE16")]
    SA2Large16, // 4C16G

    #[strum(serialize = "MA3.LARGE32")]
    MA3Large32, // 4C32
    #[strum(serialize = "MA2.LARGE32")]
    MA2Large32, // 4C32
    #[strum(serialize = "M5.LARGE32")]
    M5Large32, // 4C32

    #[strum(serialize = "SA2.2XLARGE32")]
    SA22Xlarge32, //8C32G
}