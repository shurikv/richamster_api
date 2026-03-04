#[macro_export]
macro_rules! send_request {
    ( $url:expr, $method:expr ) => {{ $crate::api::CCLIENT.request($method, $url).send().await? }};
    ( $url:expr, $method:expr, $auth_state:expr ) => {{
        $crate::api::CLIENT
            .request($method, $url)
            .compose(&$auth_state)
            .send()
            .await?
    }};
    ( $url:expr, $method:expr, $auth_state:expr, $body:expr) => {{
        $crate::api::CLIENT
            .request($method, $url)
            .body($body.clone())
            .header("Content-Type", "application/json")
            .compose_with_payload(&$auth_state, $body.as_str())
            .send()
            .await?
    }};
}

#[macro_export]
macro_rules! prepare_request {
    ( $url:expr, $request_type:ident) => {{ $crate::api::CCLIENT.$request_type($url) }};
    ( $url:expr, $payload:expr, $request_type:ident) => {{
        $crate::api::CCLIENT
            .$request_type($url)
            .body($payload.clone())
            .header("Content-Type", "application/json")
    }};
}

#[macro_export]
macro_rules! process_response {
    ( $response:expr, $de_type:ty ) => {{
        match $response.status() {
            StatusCode::OK => {
                let res = $response.text().await?;
                let response: $de_type = serde_json::from_str(res.as_str())?;
                Ok(response)
            }
            StatusCode::UNAUTHORIZED => Err(RichamsterError::UnauthorizedAccess),
            status => {
                let str = $response.text().await?;
                Err(RichamsterError::UnsupportedResponseCode(status, str))
            }
        }
    }};
}
