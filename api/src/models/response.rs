#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody<T> {
    pub message: String,
    pub data: T,
}

impl<T> ResponseBody<T> {
    pub fn new(message: &str, data: T) -> ResponseBody<T> {
        ResponseBody {
            message: message.to_string(),
            data,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBalance<T> {
    pub data: T,
}

impl<T> ResponseBalance<T> {
    pub fn new(data: T) -> ResponseBalance<T> {
        ResponseBalance { data }
    }
}
