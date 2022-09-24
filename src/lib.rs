use chrono::{Date, Datelike, NaiveDate, TimeZone, Utc};
use rand::Rng;

/// Date of first ever comic
const FIRST_DATE: &str = "1978-06-19";
/// Url of comic, without date
const BASE_URL: &str = "https://cors.bridged.cc/https://www.gocomics.com/garfield/";

/// Get random `Date<Utc>` between `FIRST_DATE` and current date (`Utc::now()`)
pub fn random_date() -> Date<Utc> {
  Utc
    .timestamp(
      rand::thread_rng().gen_range(
        NaiveDate::parse_from_str(FIRST_DATE, "%Y-%m-%d")
          .expect("First date not properly formatted")
          .and_hms(0, 0, 0)
          .timestamp()..=Utc::now().timestamp(),
      ),
      0,
    )
    .date()
}

/// Get today's date as `Date<Utc>`
pub fn today_date() -> Date<Utc> {
  Utc::now().date()
}

/// Fetch url of comic image file from `Date<Utc>`
pub async fn get_comic_url(date: Date<Utc>) -> Result<String, String> {
  let url = BASE_URL.to_string() + &date_to_string(date, "/", false);
  let body = match fetch(&url).await {
    Ok(url) => url,
    Err(err) => return Err(err.to_string()),
  };
  let index = match body.find("https://assets.amuniversal.com") {
    Some(i) => i,
    None => return Err("Cannot find image in body".to_string()),
  };
  Ok(body[index..index + 63].to_string())
}

/// Convert `Date<Utc>` to string separated by given character
pub fn date_to_string(date: Date<Utc>, sep: &str, fill: bool) -> String {
  let month = date.month();
  let day = date.day();
  date.year().to_string()
    + sep
    + if fill && month < 10 { "0" } else { "" }
    + &month.to_string()
    + sep
    + if fill && day < 10 { "0" } else { "" }
    + &day.to_string()
}

/// Quick fetch function
async fn fetch(url: &str) -> Result<String, reqwest::Error> {
  reqwest::Client::new()
    .get(url)
    .header(
      "x-cors-grida-api-key",
      "77a0175b-4435-49b0-ad18-52d2dea5a548",
    )
    .send()
    .await?
    .text()
    .await
}
