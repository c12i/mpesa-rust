extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(MpesaSecurity)]
pub fn mpesa_security_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Error parsing input");

    impl_mpesa_security(&ast)
}

fn impl_mpesa_security(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        use openssl::x509::X509;
        use openssl::rsa::Padding;
        use base64::encode;
        use std::error::Error;
        use crate::MpesaError;

        impl MpesaSecurity for #name {
            fn gen_security_credentials(&self) -> Result<String, MpesaError> {
                let pem = self.environment().get_certificate().as_bytes();
                let cert = X509::from_pem(pem)?;
                // getting the public and rsa keys
                let pub_key = cert.public_key()?;
                let rsa_key = pub_key.rsa()?;
                // configuring the buffer
                let buf_len = pub_key.size();
                let mut buffer = vec![0; buf_len];

                rsa_key.public_encrypt(
                    self.initiator_password().as_bytes(),
                    &mut buffer,
                    Padding::PKCS1,
                )?;
                Ok(encode(buffer))
            }
        }
    };
    gen.into()
}
