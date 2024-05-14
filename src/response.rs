#[derive(Debug, serde::Deserialize, serde::Serialize)]
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

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[allow(non_snake_case)]
pub struct Response<T> {
    pub status: Status,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub requestUrl: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub requestMethod: Option<String>,
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
