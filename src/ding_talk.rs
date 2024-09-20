#![allow(dead_code)]
// https://open.dingtalk.com/document/orgapp/robot-reply-and-send-messages
use hmac::{Hmac, Mac}; 
use reqwest::Client;
use serde::Serialize;
use sha2::Sha256;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use urlencoding::encode as url_encode;

// 定义图文消息的结构体
#[derive(Serialize)]
#[allow(non_snake_case)] // 忽略非 snake_case 命名警告

struct Link {
    title: String,
    text: String,
    messageUrl: String,
    picUrl: String,
}

// 定义 Markdown 消息的结构体
#[derive(Serialize)]
#[allow(non_snake_case)] // 忽略非 snake_case 命名警告
struct Markdown {
    title: String,
    text: String,
}


// 定义普通文本消息的结构体
#[derive(Serialize)]
struct Text {
    content: String,
}


// 定义消息的结构体，可以包含不同类型的消息
#[derive(Serialize)]
#[serde(untagged)]
enum Message {
    Link { msgtype: String, link: Link },
    Markdown { msgtype: String, markdown: Markdown },
    Text { msgtype: String, text: Text },
}

pub struct DingTalk {
    webhook_url: String,
    secret: String, // 钉钉机器人的秘钥
}

impl DingTalk {
    // 创建一个新的 DingTalk 实例
    pub fn new(webhook_url: &str, secret: &str) -> Self {
        DingTalk {
            webhook_url: webhook_url.to_string(),
            secret: secret.to_string(),
        }
    }

    // 发送图文消息
    pub async fn send_link_message(
        &self,
        title: &str,
        text: &str,
        message_url: &str,
        pic_url: &str,
    ) -> Result<(), Box<dyn Error>> {
        let message = Message::Link {
            msgtype: "link".to_string(),
            link: Link {
                title: title.to_string(),
                text: text.to_string(),
                messageUrl: message_url.to_string(),
                picUrl: pic_url.to_string(),
            },
        };

        self.send_message(message).await
    }

    // 发送 Markdown 消息
    pub async fn send_markdown_message(
        &self,
        title: &str,
        text: &str,
    ) -> Result<(), Box<dyn Error>> {
        let message = Message::Markdown {
            msgtype: "markdown".to_string(),
            markdown: Markdown {
                title: title.to_string(),
                text: text.to_string(),
            },
        };

        self.send_message(message).await
    }


     // 发送普通文本消息
     pub async fn send_text_message(
        &self,
        content: &str,
    ) -> Result<(), Box<dyn Error>> {
        let message = Message::Text {
            msgtype: "text".to_string(),
            text: Text {
                content: content.to_string(),
            },
        };

        self.send_message(message).await
    }

    // 统一发送消息的内部函数
    async fn send_message(&self, message: Message) -> Result<(), Box<dyn Error>> {
        // 获取签名后的 webhook URL
        let signed_url = self.get_signed_url()?;

        let client = Client::new();
        let response = client.post(&signed_url)
            .json(&message)
            .send()
            .await?;

        if response.status().is_success() {
            println!("消息发送成功");
        } else {
            println!("消息发送失败: {:?}", response.text().await?);
        }

        Ok(())
    }

    // 生成带签名的 URL
    fn get_signed_url(&self) -> Result<String, Box<dyn Error>> {
        // 计算当前时间戳（毫秒级）
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_millis()
            .to_string();

        // 构建要签名的字符串（timestamp + "\n" + secret）
        let string_to_sign = format!("{}\n{}", timestamp, self.secret);

        // 使用 HMAC-SHA256 进行签名
        let mut mac = Hmac::<Sha256>::new_from_slice(self.secret.as_bytes())?;
        mac.update(string_to_sign.as_bytes());
        let signature = mac.finalize().into_bytes();

        // 将签名结果编码为 base64
        let encoded_signature = base64::encode(signature);

        // 将签名和时间戳附加到 webhook_url 中
        let signed_url = format!(
            "{}&timestamp={}&sign={}",
            self.webhook_url, timestamp, url_encode(&encoded_signature)
        );

        Ok(signed_url)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 创建 DingTalk 实例
    let ding_talk = DingTalk::new("your_webhook_url", "your_secret");

    // 发送图文消息
    ding_talk.send_link_message(
        "这是标题",
        "这是图文消息的内容部分",
        "https://example.com",
        "https://example.com/image.jpg"
    ).await?;

    // 发送 Markdown 消息
    ding_talk.send_markdown_message(
        "Markdown 标题",
        "这是 **Markdown** 消息的内容部分"
    ).await?;

    Ok(())
}
