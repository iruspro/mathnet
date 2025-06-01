// region:    --- Modules

use gloo_net::{
    Error,
    http::{Method, RequestBuilder, Response},
};
use sauron::web_sys::console;
use serde::Serialize;

// endregion: --- Modules

pub async fn do_post<B>(url: &str, body: B) -> Result<Response, Error>
where
    B: Serialize + std::fmt::Debug,
{
    // For early development only
    console::log_1(&format!("request to {url} with {:?}", body).into());

    let request_builder = RequestBuilder::new(url).method(Method::POST);
    let request = request_builder.json(&body)?;
    let response = request.send().await?;

    Ok(response)
}
