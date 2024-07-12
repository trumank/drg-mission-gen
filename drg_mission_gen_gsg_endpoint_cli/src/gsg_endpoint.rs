use thiserror::Error;

use crate::deep_dive_response::DeepDiveResponse;

const GSG_DEEP_DIVE_ENDPOINT: &str = "https://drg.ghostship.dk/events/deepdive";

#[derive(Debug, Error)]
pub(crate) enum EndpointError {
    #[error("request to `{url}` failed ({status_code}): {status_text}")]
    RequestFailed {
        url: &'static str,
        status_code: u16,
        status_text: String,
        response_body: String,
    },
    #[error("response failed to deserialize into expected `DeepDiveResponse` JSON format")]
    FailedToDeserialize,
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("ureq error: {0}")]
    GenericUreqFailure(String),
}

pub(crate) fn query_gsg_deep_dive_endpoint() -> Result<DeepDiveResponse, EndpointError> {
    let raw_response = match ureq::get(GSG_DEEP_DIVE_ENDPOINT).call() {
        Ok(raw_response) => raw_response,
        Err(ureq::Error::Status(code, response)) => {
            let status_text = response.status_text().to_string();
            let res_body = response.into_string()?;
            return Err(EndpointError::RequestFailed {
                url: GSG_DEEP_DIVE_ENDPOINT,
                status_code: code,
                status_text: status_text,
                response_body: res_body,
            });
        }
        Err(other) => return Err(EndpointError::GenericUreqFailure(other.to_string())),
    };

    raw_response
        .into_json::<DeepDiveResponse>()
        .map_err(|_| EndpointError::FailedToDeserialize)
}
