use twilight_http::Client as HttpClient;

pub struct Context {
    pub http: HttpClient,
}

impl Context {
    pub fn new(http: HttpClient) -> Self {
        Context { http: http }
    }
}
