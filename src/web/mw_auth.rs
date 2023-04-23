use async_trait::async_trait;
use axum::{http::{Request, request::Parts}, response::Response, middleware::Next, extract::FromRequestParts, RequestPartsExt};
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

use crate::{web::AUTH_TOKEN, Error, ctx::Ctx};

pub async fn mw_require_auth<B>(
    ctx: Result<Ctx, Error>,
    req: Request<B>, 
    next: Next<B>
) -> Result<Response, Error> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");

    ctx?;

    // TODO: Token componenets validation.

    Ok(next.run(req).await)
}

// region: -- Ctx Extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Error> {
        println!("->> {:<12} - Ctx", "EXTRACTOR");

        let cookies = parts.extract::<Cookies>().await.unwrap();

        let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

        let (user_id, exp, sign) = auth_token
            .ok_or(Error::AuthFailNoAuthTokenCookie)
            .and_then(parse_token)?;

        Ok(Ctx::new(user_id))

    }
}
// endregion: --- Ctx Extractor


/// Parse a token of format 'user-[user-id].[expiration].[signature]'
/// Returns (user_id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String), Error> {
    let (_whole, user_id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#, // a literal regex
        &token
    )

    .ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}