
use xdnmb_rs::*;

use cookie::UserCookie;


#[allow(unused)]
const COOKIE_HASH: &str= "%01%02%0304%A0%A1%AAh%AA%AA%AAb%AA%AA%AAH%AA%AA%AAA%AA%AA%AA";
const FEED_UUID: &str = "你的收藏UUID";



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // Example usage
    let cookie = UserCookie::new("test", COOKIE_HASH);
    let mut client = ApiClient::new(
        Some(cookie),
        Some(FEED_UUID.to_string()),
    );
    client.init().await?;
    // let forum_list = client.get_forum_list().await.unwrap();
    // println!("获取版块：{forum_list:?}");
    // println!("forum at index 0 's msg: {}", forum_list[0].forums[0].msg);
    let forum_content = client.get_threads_from_forum(17, 0).await?;
    let thread= &forum_content[0];
    println!("No.{}  {} ID: {}\n{}", thread.rid, thread.now, thread.user_hash, thread.content);
    // let thread = client.get_thread(thread.rid, None, true).await?;
    for reply in thread.replies.iter() {
        println!("{}", reply.content);
    }
    print!("\n\n\n\n\n");
    for reply in client.get_thread_page(thread.rid.as_str(), 1, false).await?.replies.iter() {
        println!("{}", reply.content);
    }
    for thread in client.get_threads_from_feed(1).await?.iter() {
        println!("{thread:?}\n----------\n");
    }

    // println!("{thread:?}");
    Ok(())
}