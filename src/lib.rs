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
    pub async fn run(&self) {
        let mut tasks = vec![];
        for ticker in &self.ticker_list {
            tasks.push(self.query_ticker(&ticker))
        }
        // wait and return all futures
        let results = join_all(tasks).await;
        println!("{:?}", results);
    }

    async fn query_ticker(&self, ticker: &str) -> Result<String, reqwest::Error>{
        let url = self.base_url.to_owned() + ticker;
        let response  = reqwest::get(&url).await.unwrap();
        //response.json::<Body>().await
        response.text().await

    }
}

pub fn test() {
    let downloader = Downloader::new(vec!["TSLA", "MSFT", "A", ]);
    downloader.run();
}