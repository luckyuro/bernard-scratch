use reqwest::Error;
use std::{error, fmt, io};

#[derive(Copy, Clone, Debug)]
pub struct ScrapError;

impl fmt::Display for ScrapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error While Scrapping this page.")
    }
}
impl error::Error for ScrapError {}
impl From<reqwest::Error> for ScrapError {
    fn from(_: Error) -> Self {
        Self
    }
}

impl From<io::Error> for ScrapError {
    fn from(_: io::Error) -> Self {
        Self
    }
}

pub async fn load_page(url: &str) -> Result<String, ScrapError> {
    Ok(reqwest::get(url).await?.text().await?)
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
