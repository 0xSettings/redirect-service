// It looks like `service` might not be correctly resolved. You might need to double-check the module path or mod imports.
// For now, assuming the crate/module is set up correctly:

use service::url_service::UrlService;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Your main async logic goes here
    Ok(())
}
}
