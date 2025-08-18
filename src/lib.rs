use reqwest::multipart;
use serde_json as json;
use tokio;
use std::{collections::HashMap, error::Error, fmt::Display};


pub mod forum; use forum::{ ForumList, ThreadList, TimelineList, ThreadReply};
pub mod cdnpath; use cdnpath::CdnPathList;
pub mod cookie; use cookie::UserCookie;



#[derive(Clone, Debug)]
pub struct ApiClient {
    pub auth_cookie: Option<UserCookie>,
    pub feed_uuid: Option<String>,
    client: reqwest::Client,
    base_url: String,
    cdn_path_list: Option<cdnpath::CdnPathList>,
}

const BASE_URL: &str = "https://api.nmb.best";


impl ApiClient {

    // 初始化对象
    pub fn new(auth_cookie: Option<UserCookie>, feed_uuid: Option<String>) -> Self {
        ApiClient {
            auth_cookie,
            feed_uuid,
            client: reqwest::Client::new(),
            base_url: BASE_URL.to_string(),
            cdn_path_list: None,
        }
    }

    pub async fn init(&mut self) -> Result<(), Box<dyn Error>> {
        let cdn_path_list = self.get_cdn_path().await?;
        self.cdn_path_list = Some(cdn_path_list);
        Ok(())
    }

    async fn api_get(&self, api_path: &str, params: Option<HashMap<&str, &str>>) -> Result<json::Value, Box<dyn Error>> {
        let url = format!("{}/{}", self.base_url, api_path);
        let mut request = self.client.get(url);
        let cookie;
        if let Some(c) = self.auth_cookie.as_ref() {
            cookie = &c.value;
            request = request.header(reqwest::header::COOKIE, cookie);
        }
        if let Some(params) = params {
            request = request.query(&params);
        }
        let response = request.send().await?;
        let json: json::Value = response.json().await?;
        Ok(json)
    }

    // 获取图片CDN地址
    // 图片链接：{cdn_path}/image/{日期路径}/{十六进制编号}.{扩展名}
    // 实际的图片地址由 CDN 地址和 img、ext 两个字段组合而成。
    // 例如：图片 CDN 地址为 https://image.nmb.best/，img 为 2022-06-18/62acedc59ef24，ext 为 .png，
    // 则图片地址为 https://image.nmb.best/image/2022-06-18/62acedc59ef24.png，
    // 缩略图地址为 https://image.nmb.best/thumb/2022-06-18/62acedc59ef24.png。
    // https://github.com/TransparentLC/xdcmd/wiki/%E8%87%AA%E5%B7%B1%E6%95%B4%E7%90%86%E7%9A%84-X-%E5%B2%9B%E5%8C%BF%E5%90%8D%E7%89%88-API-%E6%96%87%E6%A1%A3#%E5%85%B6%E4%BB%96%E7%9A%84%E8%AF%B4%E6%98%8E-1
    async fn get_cdn_path(&self) -> Result<CdnPathList, Box<dyn Error>> {
        let api_path = "api/getCDNPath";
        let json = self.api_get(api_path, None).await?;
        let cdn_path_list = serde_json::from_value::<CdnPathList>(json)?;
        Ok(cdn_path_list)
    }

    // 获取板块列表
    pub async fn get_forum_list(&self) -> Result<ForumList, Box<dyn Error>> {
        let api_path = "api/getForumList";
        let json = self.api_get(api_path, None).await?;
        let forum_list = serde_json::from_value::<ForumList>(json)?;
        Ok(forum_list)
    }

    // 获取时间线列表
    pub async fn get_timeline_list(&self) -> Result<TimelineList, Box<dyn Error>> {
        let api_path = "api/getTimelineList";
        let json = self.api_get(api_path, None).await?;
        let timeline_list = serde_json::from_value::<TimelineList>(json)?;
        Ok(timeline_list)
    }

    // 查看版面，fid为版面ID，page为页数（可置空）
    async fn get_threads(
        &self,
        api_path: &str,
        id: Option<&str>,
        page: Option<&str>,
    ) -> Result<ThreadList, Box<dyn Error>> {
        let mut params = HashMap::new();
        if let Some(id) = id{
            params.insert("id", id);
        }
        if let Some(page) = page {
            params.insert("page", page);
        }
        let params = match params.is_empty() {
            false => Some(params),
            true => None,
        };
        let json = self.api_get(api_path, params).await?;
        let thread_list = serde_json::from_value::<ThreadList>(json)?;
        Ok(thread_list)
    }

    #[inline]
    pub async fn get_threads_from_forum<FID, NUM>(
        &self,
        fid: FID,
        page: NUM,
    ) -> Result<ThreadList, Box<dyn Error>>
        where
            FID: Display,
            NUM: Display,
    {
        self.get_threads(
            "api/showf",
            Some(fid.to_string().as_str()),
            Some(page.to_string().as_str()),
        ).await // （showf非用于查看时间线）
    }

    #[inline]
    pub async fn get_threads_from_timeline<TLID, NUM>(
        &self,
        tlid: TLID,
        page: NUM,
    ) -> Result<ThreadList, Box<dyn Error>>
        where
            TLID: Display,
            NUM: Display,
    {
        self.get_threads(
            "api/timeline",
            Some(tlid.to_string().as_str()),
            Some(page.to_string().as_str()),
        ).await
    }

    // 查看串，id为串号，page为页数
    pub async fn get_thread_page<TID, NUM>(
        &self,
        tid: TID,
        page: NUM,
        po_only: bool,
    ) -> Result<forum::Thread, Box<dyn Error>>
        where
            TID: Display,
            NUM: Display,
    {
        let api_path = match po_only {
            false =>"api/thread",
            true => "api/po",
        };
        let mut params = HashMap::new();
        let rid = tid.to_string();
        params.insert("id", rid.as_str());
        let page = page.to_string();
        params.insert("page", page.as_str());
        let json = self.api_get(api_path, Some(params)).await?;
        let thread = serde_json::from_value::<forum::Thread>(json)?;
        Ok(thread)
    }

    pub async fn get_reply<TID>(&self, tid: TID) -> Result<ThreadReply, Box<dyn Error>>
        where TID: Display
    {
        let api_path = "api/ref";
        let rid = tid.to_string();
        let params: [(&'static str, &str); 1] = [("id", rid.as_str())];
        let json = self.api_get(api_path, Some(params.into())).await?;
        let reply = serde_json::from_value::<ThreadReply>(json)?;
        Ok(reply)
    }

    // 发新串
    pub async fn post_new_thread<FID>(
        &self,
        fid: FID,
        title: Option<&str>,
        name: Option<&str>,
        email: Option<&str>,
        content: Option<&str>,
        img_filepath: Option<&str>,
        img_watermark: Option<bool>,
    ) -> Result<String, Box<dyn Error>>
        where
            FID: Display,
    {
        let action_url = "https://www.nmbxd1.com/Home/Forum/doPostThread.html";
        let mut form = multipart::Form::new()
            .text("fid", fid.to_string());

        if let Some(t) = title {
            form = form.text("title", t.to_string());
        }
        if let Some(n) = name {
            form = form.text("name", n.to_string());
        }
        if let Some(e) = email {
            form = form.text("email", e.to_string());
        }
        if let Some(c) = content {
            form = form.text("content", c.to_string());
        }

        if let Some(img_filepath) = img_filepath {
            let file_content = tokio::fs::read(img_filepath).await?;
            let file_name = std::path::Path::new(img_filepath)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("file")
                .to_string();
            let part = multipart::Part::bytes(file_content).file_name(file_name);
            form = form.part("image", part);
        }
        if let Some(true) = img_watermark {
            form = form.text("water", "true");
        }

        let mut request = self.client.post(action_url).multipart(form);
        if let Some(cookie) = self.auth_cookie.as_ref() {
            let cookie_header = format!("userhash={}", cookie.value);
            request = request.header("cookie", cookie_header);
        }

        let res = request.send().await?;
        let text = res.text().await?;
        if text.contains("<h1>:)</h1>") {
            Ok(String::default())
        } else {
            Err(text.into())
        }
    }

    // 发评论
    pub async fn post_thread_reply<TID>(
        &self,
        tid: TID,
        title: Option<&str>,
        name: Option<&str>,
        email: Option<&str>,
        content: Option<&str>,
        img_filepath: Option<&str>,
        img_watermark: Option<bool>,
    ) -> Result<String, Box<dyn Error>>
        where
            TID: Display,
    {
        let action_url = "https://www.nmbxd1.com/Home/Forum/doReplyThread.html";
        let mut form = multipart::Form::new()
            .text("resto", tid.to_string());

        if let Some(t) = title {
            form = form.text("title", t.to_string());
        }
        if let Some(n) = name {
            form = form.text("name", n.to_string());
        }
        if let Some(e) = email {
            form = form.text("email", e.to_string());
        }

        if let Some(c) = content {
            form = form.text("content", c.to_string());
        }

        if let Some(img_filepath) = img_filepath {
            let file_content = tokio::fs::read(img_filepath).await?;
            let file_name = std::path::Path::new(img_filepath)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("file")
                .to_string();
            let part = multipart::Part::bytes(file_content).file_name(file_name);
            form = form.part("image", part);
        }
        if let Some(true) = img_watermark {
            form = form.text("water", "true");
        }

        let mut request = self.client.post(action_url).multipart(form);
        if let Some(cookie) = self.auth_cookie.as_ref() {
            let cookie_header = format!("userhash={}", cookie.value);
            request = request.header("cookie", cookie_header);
        }

        let res = request.send().await?;
        let text = res.text().await?;
        if text.contains("<h1>:)</h1>") {
            Ok(String::default())
        } else {
            Err(text.into())
        }
    }

    // 查看订阅，uuid为订阅id，page为页数（可置空）
    pub async fn get_threads_from_feed<NUM>(&self, page: NUM) -> Result<ThreadList, Box<dyn Error>>
        where NUM: Display
    {
        let api_path = "api/feed";
        let feed_uuid = self.feed_uuid.as_ref().unwrap();
        let page = page.to_string();
        let params: [(&'static str, &str); 2] = [("uuid", feed_uuid.as_str()), ("page", page.as_str())];
        let json = self.api_get(api_path, Some(params.into())).await?;
        let thread_list = serde_json::from_value::<ThreadList>(json)?;
        Ok(thread_list)
    }

    // 添加订阅，uuid为订阅id，rid为串号
    pub async fn add_feed(
        &self,
        uuid: &str,
        tid: &str,
    ) -> Result<json::Value, Box<dyn Error>> {
        let url = format!("{}/api/addFeed?uuid={}", self.base_url, uuid);
        let params = [("tid", tid)];
        let res = self.client.post(&url).form(&params).send().await?;
        let json: json::Value = res.json().await?;
        Ok(json)
    }

    // 删除订阅，uuid为订阅id，rid为串号
    pub async fn del_feed(
        &self,
        uuid: &str,
        tid: &str,
    ) -> Result<json::Value, Box<dyn Error>> {
        let url = format!("{}/api/delFeed?uuid={}", self.base_url, uuid);
        let params = [("tid", tid)];
        let res = self.client.post(&url).form(&params).send().await?;
        let json: json::Value = res.json().await?;
        Ok(json)
    }

}