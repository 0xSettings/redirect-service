
//business use cases
pub struct UrlService;

impl UrlService{
    //shorten long url reuse if it exist
    pub fn short_url(original_url: &str) -> Result<Url, DomainError> {
        //look up existing key
        if let Some(existing_key) = KeyService::get_existing_key(original_url) {
            OK(Url::new(existing_key, original_url.to_string()));
        }

        //
    }
}


//validate input

//call key service to generate a short key

//construct the model entity
//call repository to save the url
//return the response
//handle errors collisions retries