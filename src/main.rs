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
        match ding_talk.send_markdown_message(&format!("{}:{}",address,title),&format!("{}","Assert: ![screenshot](data:image/jpeg;base64,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)")).await {
            Ok(_) => println!("消息发送成功"),
            Err(e) => eprintln!("消息发送失败: {:?}", e),
        }
    // }); 
    // 等待钉钉消息发送完成
    // send_task.await.expect("发送未完成");


    driver.quit().await?;
    Ok(())
}