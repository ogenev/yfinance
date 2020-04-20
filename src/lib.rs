use reqwest::Client;

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
    pub async fn run(&self) {
        let mut tasks = vec![];
        let client = Client::new();
        for ticker in &self.ticker_list {
            let url = self.base_url.to_owned() + ticker;
            tasks.push(client.get(&url).send().await.unwrap().text().await.unwrap());

        }
        println!("{:?}", tasks);
    }
}


pub fn test() {
    let downloader = Downloader::new(vec!["TSLA", "MSFT", "A"]);
    downloader.run();
}