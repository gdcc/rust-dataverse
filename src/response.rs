#[derive(Debug, serde::Deserialize)]
pub enum Status {
    OK,
    ERROR,
}

impl Status {
    pub fn as_str(&self) -> &str {
        match self {
            Status::OK => "OK",
            Status::ERROR => "ERROR",
        }
    }

    pub fn is_ok(&self) -> bool {
        match self {
            Status::OK => true,
            Status::ERROR => false,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Response<T> {
    pub status: Status,
    pub data: Option<T>,
    pub message: Option<String>,

    #[serde(rename = "requestUrl")]
    pub request_url: Option<String>,

    #[serde(rename = "requestMethod")]
    pub request_method: Option<String>,
}

impl<T> Response<T> {
    pub fn is_ok(&self) -> &Response<T> {
        match self.status {
            Status::ERROR => {
                panic!("Error: {}", self.message.as_ref().unwrap())
            }
            _ => self,
        }
    }
}
