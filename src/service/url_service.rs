
//business use cases
pub struct UrlService;

pub fn create_url(original_url: String) -> Result<UrlResponse, DomainError> {
    //validate input & generate short key
    
    let short_key = ShortKey::new(original_url);
    let url = Url::new(short_key, original_url);
    //repo save url

}
//validate input

//call key service to generate a short key

//construct the model entity
//call repository to save the url
//return the response
//handle errors collisions retries