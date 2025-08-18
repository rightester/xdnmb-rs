use std::ops::Deref;
use std::fmt::Display;

use serde::{ Serialize };
use serde::{ Deserialize, Deserializer };
use serde_json::Value;
use serde::de::DeserializeOwned;







// 类型别名，代表论坛组的列表
#[allow(unused)]
pub type ForumList = Vec<ForumGroup>;


type NUM = SNum;
type BOOL = SNBool;
type TIME = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForumGroup {
    pub forums: Vec<Forum>,
    pub id: NUM,
    pub name: String,
    pub sort: NUM,
    pub status: String,
}


// 代表单个版块（论坛）
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Forum {

    #[serde(rename="id")]
    pub fid: NUM, // 数字

    pub msg: String, // HTML内容

    pub name: String,

    pub auto_delete: Option<BOOL>,

    #[serde(rename = "createdAt")]
    pub created_at: Option<TIME>, // 可以考虑用 chrono::NaiveDateTime

    pub fgroup: Option<NUM>,

    pub forum_fuse_id: Option<NUM>,

    pub interval: Option<NUM>, // 版面发文间隔时间（秒数）

    pub permission_level: Option<BOOL>,

    pub safe_mode: Option<BOOL>,

    #[serde(rename = "showName")]
    pub show_name: Option<String>,

    pub sort: Option<NUM>, // 数字

    pub status: Option<String>, // 单小写字符

    pub thread_count: Option<NUM>, // 数字

    #[serde(rename = "updateAt")]
    pub update_at: Option<String>,
}


#[allow(unused)]
pub type ThreadList = Vec<Thread>;


#[allow(unused)]
pub type TimelineList = Vec<TimelineForum>;


#[derive(Deserialize, Serialize, Debug)]
pub struct TimelineForum {

    #[serde(rename="id")]
    tid: NUM,

    name: String,

    display_name: String,

    notice: String,

    max_page: NUM,
}


/// 代表论坛中的一个主题串（主贴）及其回复。
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Thread {


    /// 主串的串号。
    #[serde(rename = "id")]
    pub tid: NUM,
    /// Po的饼干
    pub user_hash: String,
    /// 发布时间的格式化字符串。
    pub now: TIME, // 格式： "2025-07-31(四)13:49:32"，可能没有中间的星期和括号而是一个空格

    /// 主串所属的版块ID。
    pub fid: Option<NUM>,
    /// 该主题串的回复数量
    #[serde(alias = "ReplyCount")]
    pub reply_count: Option<NUM>,

    /// 帖子标题
    pub title: Option<String>,
    /// 作者笔名
    pub name: Option<String>,
    /// 留个邮箱
    pub email: Option<String>,

    /// 帖子正文，含HTML
    pub content: String,
    /// 附图文件在服务器上的路径名
    pub img: String,
    /// 附图文件类型扩展名
    pub ext: String,

    /// 主串的一页回复列表
    #[serde(rename = "Replies")]
    pub replies: Option<Vec<ThreadReply>>,

    /// 是否应被Sage
    pub sage: Option<BOOL>,
    /// 是否为特权帖
    pub admin: Option<BOOL>,
    /// 是否应被隐藏
    #[serde(alias = "Hide")]
    pub hide: Option<BOOL>,

    /// 网页版除去显示的最近几条回复后剩余的回复数量 “回应有……篇被省略。要阅读所有回应请按下回应链接。”
    #[serde(rename = "RemainReplies")]
    pub remain_replies: Option<NUM>, // 有此字段则表示当前Thread对象的回复数量是brief的，不是完整的

    /// 最近回复的回复楼串号
    #[serde(default, deserialize_with = "deserialize_string_wrapped_json")]
    pub recent_replies: Option<Vec<NUM>>, // 有此字段则表示该帖子来源为订阅列表

    // pub po: Option<String>,
    // pub user_id: Option<NUM>,
    // pub file_id: Option<NUM>,
    // pub category: Option<String>,

}

pub type ThreadReply = Thread;




/// 代表一条回复（跟帖）。

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct ThreadReply {

//     /// 回复的唯一串号。
//     #[serde(rename = "id")]
//     pub rid: NUM,
//     pub user_hash: String,
//     pub now: TIME,

//     /// 回复的主串所属版块ID。
//     pub fid: Option<NUM>,
//     #[serde(rename = "ReplyCount")]
//     pub reply_count: Option<NUM>,

//     pub title: String,
//     pub name: String,
//     pub email: Option<String>,

//     pub content: String,
//     pub img: String,
//     pub ext: String,

//     pub sage: Option<BOOL>,
//     pub admin: BOOL,
//     #[serde(rename = "Hide")]
//     pub hide: Option<BOOL>,
// }








// 对于JSON的值本应是一个JSON对象但却是一个字符串，需要额外处理时所用的解析函数
fn deserialize_string_wrapped_json<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: DeserializeOwned,
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    // println!("{value:?}");
    match value {
        Value::String(s) => {
            // 尝试将字符串解析为指定类型
            match s.as_str() {
                "" => Ok(None),
                _ => serde_json::from_str(&s).map_err(serde::de::Error::custom),
            }
        }
        Value::Null => {
            Ok(None)
        }
        _ => {
            // 如果不是字符串，直接尝试转换为目标类型
            serde_json::from_value(value).map_err(serde::de::Error::custom)
        }
    }
}



#[derive(Serialize, Debug, Clone, Copy)]
pub struct SNum(i64);
impl SNum {
    pub fn into_inner(self) -> i64 {
        self.0
    }
}
impl Display for SNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl Deref for SNum {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'de> Deserialize<'de> for SNum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        // println!("{value:?}");
        let number = match value {
            Value::Number(n) => {
                n.as_i64().ok_or_else(|| {
                    serde::de::Error::custom("invalid number format")
                })?
            }
            Value::String(s) => {
                match s.as_str() {
                    "" => 0,
                    _ => s.parse::<i64>().map_err(serde::de::Error::custom)?,
                }
            }
            _ => {
                return Err(serde::de::Error::custom(
                    "expected number or string"
                ));
            }
        };
        Ok(SNum(number))
    }
}

#[derive(Serialize, Debug, Clone, Copy)]
pub struct SNBool(bool);
impl SNBool {
    pub fn into_inner(self) -> bool {
        self.0
    }
}
impl Display for SNBool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl<'de> Deserialize<'de> for SNBool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        // println!("{value:?}");
        let boolnum = match value {
            Value::Bool(b) => {
                return Ok(SNBool(b));
            }
            Value::Number(num) => {
                let b = match num.as_i64().unwrap_or(-1) {
                    0 => false,
                    1 => true,
                    _ => { 
                        return Err(serde::de::Error::custom(
                            "number out of the bool convertion range"
                        ));
                    }
                };
                return Ok(SNBool(b));
            }
            Value::String(s) => {
                match s.as_str() {
                    "" => 0,
                    _ => s.parse::<i64>().map_err(serde::de::Error::custom)?,
                }
            }
            _ => {
                return Err(serde::de::Error::custom(
                    "expected bool or number or string"
                ));
            }
        };
        let bool = match boolnum {
            0 => false,
            1 => true,
            _ => {
                return Err(serde::de::Error::custom(
                    "expected bool or string"
                ));
            }
        };
        Ok(SNBool(bool))
    }
}

