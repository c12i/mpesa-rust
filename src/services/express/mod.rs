pub mod express_query;
pub mod express_request;

pub use express_query::{MpesaExpressQuery, MpesaExpressQueryBuilder, MpesaExpressQueryResponse};
pub use express_request::{
    MpesaExpress, MpesaExpressBuilder, MpesaExpressRequest, MpesaExpressResponse,
};

/// Source: [test credentials](https://developer.safaricom.co.ke/test_credentials)
pub static DEFAULT_PASSKEY: &str =
    "bfb279f9aa9bdbcf158e97dd71a467cd2e0c893059b10f78e6b72ada1ed2c919";
