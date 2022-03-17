use std::fs;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct StatusField {
    // timestamp: String,
    // error_code: usize,
    // error_message: Option<String>,
    // elapsed: usize,
    // credit_count: usize,
    // notice: Option<String>,
}
#[derive(Deserialize, Debug, Serialize)]
struct CoinQuote {
    price: f64,
    // ! #WIP
    //     "volume_24h":24842706974.843037,
    //     "volume_change_24h":7.0581,
    //     "percent_change_1h":0.08518149,
    //     "percent_change_24h":1.51555752,
    //     "percent_change_7d":-3.21378352,
    //     "percent_change_30d":-8.06423575,
    //     "percent_change_60d":-28.05417334,
    //     "percent_change_90d":-35.63420866,
    //     "market_cap":798430432199.0665,
    //     "market_cap_dominance":40.1488,
    //     "fully_diluted_market_cap":885553926299.42,
    //     "last_updated":"2022-01-19T13:36:00.000Z"
}
#[derive(Deserialize, Debug, Serialize)]
struct DataField {
    // id: usize,
    name: String,
    // symbol: String,
    // slug: String,
    // num_market_pairs: usize,
    // date_added: String,
    // is_fiat: usize,
    quote: HashMap<String, CoinQuote>
    // ! #WIP
    // "max_supply":21000000,
    // "circulating_supply":18933956,
    // "total_supply":18933956,
    // "is_active":1,
    // "platform":null,
    // "cmc_rank":1,
    // "":0,
    // "last_updated":"2022-01-19T13:36:00.000Z",
}
#[derive(Deserialize, Debug, Serialize)]
struct ApiResult {
    // status: StatusField,
    data: HashMap<String, DataField>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let request_url = format!(
        "https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest?symbol={symbols}",
        symbols = ["FIL", "BTC"].join(","),
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&request_url)
        .header("X-CMC_PRO_API_KEY", "deb827df-a1a6-4ceb-940c-3114e9adca4d")
        .send()
        .await?;

    let res = response.json::<ApiResult>().await?;

    for coin in res.data.values() {
        if let Some(quote) = coin.quote.get("USD") {
            println!("{:?}: {:?}", coin.name, quote.price);
        }
    }

    if let Ok(json) = serde_json::to_string(&res) {
        println!("{:?}", json);

        fs::write("pricer.json", json)?;
    }

    Ok(())
}
