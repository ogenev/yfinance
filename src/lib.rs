use futures::future::join_all;

pub struct Downloader<'a> {
    base_url: &'a str,
    ticker_list: Vec<&'a str>
}

impl<'a> Downloader<'a> {
    pub fn new(ticker_list: Vec<&'a str>) -> Self {
        Downloader {
            base_url: "https://query1.finance.yahoo.com/v8/finance/chart/",
            ticker_list
        }
    }
    #[tokio::main]
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut tasks: Vec<tokio::task::JoinHandle<Result<(), ()>>> = vec![];
        for ticker in &self.ticker_list {
            let url = self.base_url.to_owned() + ticker;
            tasks.push(tokio::spawn(async move {
                match reqwest::get(&url).await {
                    Ok(resp) => {
                        match resp.text().await {
                            Ok(text) => {
                                println!("RESPONSE: {} bytes from {}", text.len(), url);
                            }
                            Err(_) => println!("ERROR reading {}", url),
                        }
                    }
                    Err(_) => println!("ERROR downloading {}", url),
                }
                Ok(())
            }));
        }
        println!("Started {} tasks. Waiting...", tasks.len());
        join_all(tasks).await;
        Ok(())
    }
}

pub fn test() {
    let downloader = Downloader::new(vec!["TSLA", "MSFT", "A"]);
    downloader.run().unwrap_or_else(|error| println!("{}", error));
}