use thirtyfour::prelude::*;
use tokio::time::Duration;


#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    driver.goto("https://suivision.xyz/account/@gokalp?tab=Activity").await?;
    
    let elem_form = driver.find(By::XPath("/html/body/div/div[2]/div/main/section/div[2]/div/table/tbody")).await?; 
    tokio::time::sleep(Duration::from_secs(5)).await;
    println!("内容{:?}", elem_form.text().await?);
    driver.quit().await?;
    Ok(())
}