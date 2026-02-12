
//business use cases
pub struct UrlService;

impl UrlService{
    //shorten long url reuse if it exist
    pub fn short_url(original_url: &str) -> Result<Url, DomainError> {
        //look up existing key
        if let Some(existing_key) = KeyService::get_existing_key(original_url) {
            OK(Url::new(existing_key, original_url.to_string()));
        }

        //key generation
        let short_key = KeyService::generate_short_key(6)?;

        //save mapped key to database for easy rtrival
        KeyService::save_mapped_key(original_url, &short_key)?;
        Ok(Url::new(short_key, original_url.to_string));
    }

    //get short key resolved
    pub fn resolve_short_key(short_key: &ShortKey) -> Result<Url, DomainError> {
        url_repository::find_by_short_key(short_key)
        .map_err(|_| DomainError::UrlNotFound)
    }
}


//validate input

//call key service to generate a short key

//construct the model entity
//call repository to save the url
//return the response
//handle errors collisions retries