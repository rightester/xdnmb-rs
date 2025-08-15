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
    pub id: String,
    pub name: String,
    pub sort: String, // 注意：JSON中是字符串数字
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
    /// 主题串的唯一ID。
    #[serde(rename = "id")]
    pub rid: NUM,
    /// 主题串所属的版块ID。
    pub fid: NUM,
    /// Po的饼干
    pub user_hash: String,
    /// 帖子正文，含HTML
    pub content: String,
    /// 该主题串的直接回复数量（不包括嵌套回复）。
    #[serde(alias = "ReplyCount")]
    pub reply_count: NUM,
    /// 附图文件路径名
    pub img: String, // 可能为空字符串 ""
    /// 附图扩展名
    pub ext: String, // 可能为空字符串 ""
    /// 发布时间的格式化字符串。
    pub now: TIME, // 格式： "2025-07-31(四)13:49:32"
    /// 帖子作者笔名
    pub name: String, // 示例: "无名氏"
    /// 帖子标题
    pub title: String, // 示例: "无标题"
    /// 是否被Sage
    pub sage: Option<BOOL>, 
    /// 是否为管理员帖
    pub admin: BOOL, 
    /// 是否被隐藏
    #[serde(alias = "Hide")]
    pub hide: BOOL, 
    /// 该主题串下的回复列表（嵌套结构）。
    #[serde(rename = "Replies")]
    pub replies: Option<Vec<Reply>>,
    /// 网页版除去显示的最近几条回复后剩余的回复数量 “回应有……篇被省略。要阅读所有回应请按下回应链接。”
    #[serde(rename = "RemainReplies")]
    pub remain_replies: Option<NUM>, // 有此字段则表示当前Thread对象的回复数量是brief的，不是完整的
    #[serde(deserialize_with = "deserialize_json_string")]
    pub recent_replies: Option<Vec<NUM>>, // 有此字段则表示该帖子来源为订阅列表
    // pub po: Option<String>,
    // pub user_id: Option<NUM>,
    // pub file_id: Option<NUM>,
    // pub category: Option<String>,
    pub email: Option<String>,

}

/// 代表一条回复（跟帖）。
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reply {
    /// 回复的唯一串号。
    #[serde(rename = "id")]
    pub rid: NUM,
    /// 该回复是串头，有所属的版块ID。
    pub fid: Option<NUM>,
    /// 不知道是干啥的
    #[serde(rename = "ReplyCount")]
    pub reply_count: Option<NUM>,
    /// 附加图片的文件名（不包含扩展名和路径）。
    pub img: String, // 可能为空字符串 ""
    /// 附加图片的扩展名（例如 ".png", ".jpg"）。
    pub ext: String, // 可能为空字符串 ""
    /// 发布时间的格式化字符串。
    pub now: TIME, // 例如: "2025-07-31(四)13:49:32"
    /// 回复的饼干
    pub user_hash: String,
    /// 通常是“无名氏”的不知道啥的名称
    pub name: String,
    /// 回复标题。
    pub title: String, // 示例: "无标题"
    /// 回复正文内容，可能包含HTML。
    pub content: String,
    /// 是否启用Sage功能（回复时不顶帖）。
    pub sage: Option<BOOL>, // 0 或 1 (在JSON中为数字)
    /// 是否为管理员/版主发布的回复。
    pub admin: BOOL, // 0 或 1 (在JSON中为数字)
    /// 回复是否被隐藏。
    #[serde(rename = "Hide")]
    pub hide: Option<BOOL>, // 0 或 1 (在JSON中为数字)
}








// 对于JSON的值本应是一个JSON对象但却是一个字符串，需要额外处理时所用的解析函数
fn deserialize_json_string<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: DeserializeOwned,
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::String(s) => {
            // 尝试将字符串解析为指定类型
            match s.as_str() {
                "" => Ok(None),
                _ => serde_json::from_str(&s).map_err(serde::de::Error::custom),
            }
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
        let boolnum = match value {
            Value::Bool(b) => {
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
                    "expected number or string"
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

