pub mod express_query;
pub mod express_request;

use chrono::{DateTime, Local};
pub use express_query::{MpesaExpressQuery, MpesaExpressQueryBuilder, MpesaExpressQueryResponse};
pub use express_request::{
    MpesaExpress, MpesaExpressBuilder, MpesaExpressRequest, MpesaExpressResponse,
};

/// Source: [test credentials](https://developer.safaricom.co.ke/test_credentials)
pub static DEFAULT_PASSKEY: &str =
    "bfb279f9aa9bdbcf158e97dd71a467cd2e0c893059b10f78e6b72ada1ed2c919";

/// Helper function to serialize a `DateTime<Local>` to a string
fn serialize_utc_to_string<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let s = date.format("%Y%m%d%H%M%S").to_string();
    serializer.serialize_str(&s)
}
