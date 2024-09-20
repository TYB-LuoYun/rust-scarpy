use thirtyfour::prelude::*;
use tokio::time::Duration;

mod ding_talk;

use ding_talk::DingTalk;
 
use md5;

 
 
static mut MD5_HASH: &str = "";


#[tokio::main]
async fn main() -> WebDriverResult<()> { 
    let domain = "https://suivision.xyz/";
    let address = "@gokalp";
    // 创建一个钉钉实例，传入 webhook 地址
    let webhook_url = "https://oapi.dingtalk.com/robot/send?access_token=c9c9b1c10740df95524b5aef39e27bf1240848ac27bd1b406b4cd4a8106e74f9";
    let secret = "SEC638456b26e8209809ee536d57db0633f8fd2a8ea213dfa10425de96bf7c0c2b9"; // 替换为你的秘钥
    let ding_talk = DingTalk::new(webhook_url, secret);



    // 创建一个间隔 30 秒的定时器
    let mut interval = tokio::time::interval(Duration::from_secs(30));

    loop {
        // 等待下一个定时
        interval.tick().await;
        
        // 每隔 30 秒执行一次 `fetch_and_send_data`
        if let Err(e) = fetch_and_send_data(&domain, &address,&ding_talk).await {
            eprintln!("执行 fetch_and_send_data 时发生错误: {:?}", e);
        }
    }

    
    // Ok(());
}

  
async fn fetch_and_send_data(domain: &str, address: &str,ding_talk: &DingTalk) -> WebDriverResult<()> {
    

    let url =  &format!("{}account/{}?tab=Activity",domain,address) ;
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
   
    driver.goto(url).await?;
    tokio::time::sleep(Duration::from_secs(5)).await;
    let mut content = String::with_capacity(100); // 预分配 100 字节的空间
    let mut title = String::with_capacity(100); // 预分配 100 字节的空间

    let elem_form = driver.find(By::XPath("/html/body/div/div[2]/div/main/section/div[2]/div/table/tbody")).await?; 
   
    
    // 查找第一个 tr 元素
    let first_tr = elem_form.find(By::XPath("./tr[1]")).await?; 
    // 获取第一个 tr 元素的文本内容

    let td_1 = first_tr.find(By::XPath("./td[1]")).await?;
    let td_1_div = td_1.find(By::Css("a div .items-center")).await?;
    let _type = td_1_div.text().await?; 
    content.push_str(&format!("Address: [{}]({})      \n       ", address,url));
    content.push_str(&format!("Type: {}     \n       ", _type));

    let td_2 = first_tr.find(By::XPath("./td[2]")).await?; 
    let lis = td_2.find_all(By::Css("ul li")).await?;
    content.push_str(&format!("Assert: "));
    for li in lis {
        let _asset_amount = li.find(By::Css("div span")).await?.text().await?;
        let _asset_img = li.find(By::Css("img")).await?.attr("src").await?.unwrap_or_default();
        let _asset_coin = li.find(By::Css("a")).await?.text().await? ;
        if _asset_img.starts_with("http") {
            content.push_str(&format!("![screenshot]({}) {} {} ",_asset_img,_asset_amount,_asset_coin));
            // content.push_str(&format!("<img src='{}' alt='示例图片' width='50' height='50' style='display: block; margin: auto;'/> {} {} ",_asset_img,_asset_amount,_asset_coin));
        } else {
            content.push_str(&format!("![screenshot]({}) {} {} ",_asset_img,_asset_amount,_asset_coin));
        } 
        content.push_str(&format!(" "));

        title.push_str(&format!("{} {} ",_asset_amount,_asset_coin));
        title.push_str(&format!(" "));
    }
    content.push_str(&format!("    \n       "));
    let _interacted_with = first_tr.find(By::XPath("./td[3]")).await?.text().await?; 
    content.push_str(&format!("InteractedWith: {}     \n       ", _interacted_with));
    let _digest = first_tr.find(By::XPath("./td[4]")).await?.text().await?;
    let _digest_url = first_tr.find(By::XPath("./td[4]/a")).await?.attr("href").await?.unwrap_or_default();
    content.push_str(&format!("Digest: [{}]({}{}?tab=Changes)     \n       ", _digest,domain,_digest_url));
    let _time = first_tr.find(By::XPath("./td[6]")).await?.text().await?;
    content.push_str(&format!("Time: {}     \n       ", _time));
    println!("文本内容: {}",content); 


    
 
    let hash = md5::compute(_digest_url);
    let hash_str = format!("{:x}", hash); 
    let old_hash =unsafe { MD5_HASH };
    print!("MD5 值: {}\n", unsafe { MD5_HASH });
    
    if String::from( old_hash) == hash_str {
        println!("内容没有变化，不发送钉钉消息");
        return Ok(());
    }else{
        // 更新全局变量  
        unsafe {
            MD5_HASH = Box::leak(hash_str.into_boxed_str());
        }
    }
      

  
    // 使用 `tokio::spawn` 启动一个新的异步任务来发送钉钉消息
    // let send_task = tokio::spawn(async move { 
        match ding_talk.send_markdown_message(&format!("{}:{}",address,title),&format!("{}",&content)).await {
            Ok(_) => println!("消息发送成功"),
            Err(e) => eprintln!("消息发送失败: {:?}", e),
        }
    // }); 
    // 等待钉钉消息发送完成
    // send_task.await.expect("发送未完成");


    driver.quit().await?;
    Ok(())
}