use std::collections::HashMap;
use std::error::Error;

use crate::map;

/// Extracts access token from response as `Result<String>`.
/// 
/// # Example
/// 
/// ```
///  let map = mpesa::map!(
///     String::from("access_token") => String::from("12345abcde"),
///     String::from("expires") => String::from("infinite")
///  );
///  assert_eq!(String::from("12345abcde"), mpesa::utils::extract_auth_token(&map).unwrap());
/// ```
pub fn extract_auth_token(hm: &HashMap<String, String>) -> Result<String, Box<dyn Error>> {
    let token = hm.get("access_token").unwrap();
    Ok(token.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_extract_auth_token() {
       let map = map!(
           String::from("access_token") => String::from("12345abcde"),
           String::from("expires") => String::from("infinite")
       );

       assert_eq!(String::from("12345abcde"), extract_auth_token(&map).unwrap());
    }
}