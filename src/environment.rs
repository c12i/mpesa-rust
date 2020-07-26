///! # environment
///! Code related to setting up the desired Safaricom API environment

use std::str::FromStr;

#[derive(Debug)]
/// Enum to map to desired environment so as to access certificate
/// and the base url
/// Required to construct a new `Mpesa` struct
pub enum Environment {
    Production,
    Sandbox,
}

impl FromStr for Environment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "production" => Ok(Self::Production),
            "sandbox" => Ok(Self::Sandbox),
            _ => Err(String::from("error"))
        }
    }
}

impl Environment {
    /// Matches to intended base_url depending on Environment variant
    /// 
    /// ## Example
    /// ```
    /// use mpesa::Environment;
    /// 
    /// let env: Environment = Environment::Production;
    /// let base_url: &str = env.base_url();
    /// assert_eq!("https://api.safaricom.co.ke", base_url);
    /// ```
    pub fn base_url(&self) -> &'static str{
        match self {
            Environment::Production => "https://api.safaricom.co.ke",
            Environment::Sandbox => "https://sandbox.safaricom.co.ke",
        }
    }

    /// Match to X509 public key certificate based on
    /// environment variant
     pub fn get_certificate(&self) -> &'static str {
        match self {
            Environment::Production => r#"-----BEGIN CERTIFICATE-----
MIIGkzCCBXugAwIBAgIKXfBp5gAAAD+hNjANBgkqhkiG9w0BAQsFADBbMRMwEQYK
CZImiZPyLGQBGRYDbmV0MRkwFwYKCZImiZPyLGQBGRYJc2FmYXJpY29tMSkwJwYD
VQQDEyBTYWZhcmljb20gSW50ZXJuYWwgSXNzdWluZyBDQSAwMjAeFw0xNzA0MjUx
NjA3MjRaFw0xODAzMjExMzIwMTNaMIGNMQswCQYDVQQGEwJLRTEQMA4GA1UECBMH
TmFpcm9iaTEQMA4GA1UEBxMHTmFpcm9iaTEaMBgGA1UEChMRU2FmYXJpY29tIExp
bWl0ZWQxEzARBgNVBAsTClRlY2hub2xvZ3kxKTAnBgNVBAMTIGFwaWdlZS5hcGlj
YWxsZXIuc2FmYXJpY29tLmNvLmtlMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIB
CgKCAQEAoknIb5Tm1hxOVdFsOejAs6veAai32Zv442BLuOGkFKUeCUM2s0K8XEsU
t6BP25rQGNlTCTEqfdtRrym6bt5k0fTDscf0yMCoYzaxTh1mejg8rPO6bD8MJB0c
FWRUeLEyWjMeEPsYVSJFv7T58IdAn7/RhkrpBl1dT7SmIZfNVkIlD35+Cxgab+u7
+c7dHh6mWguEEoE3NbV7Xjl60zbD/Buvmu6i9EYz+27jNVPI6pRXHvp+ajIzTSsi
eD8Ztz1eoC9mphErasAGpMbR1sba9bM6hjw4tyTWnJDz7RdQQmnsW1NfFdYdK0qD
RKUX7SG6rQkBqVhndFve4SDFRq6wvQIDAQABo4IDJDCCAyAwHQYDVR0OBBYEFG2w
ycrgEBPFzPUZVjh8KoJ3EpuyMB8GA1UdIwQYMBaAFOsy1E9+YJo6mCBjug1evuh5
TtUkMIIBOwYDVR0fBIIBMjCCAS4wggEqoIIBJqCCASKGgdZsZGFwOi8vL0NOPVNh
ZmFyaWNvbSUyMEludGVybmFsJTIwSXNzdWluZyUyMENBJTIwMDIsQ049U1ZEVDNJ
U1NDQTAxLENOPUNEUCxDTj1QdWJsaWMlMjBLZXklMjBTZXJ2aWNlcyxDTj1TZXJ2
aWNlcyxDTj1Db25maWd1cmF0aW9uLERDPXNhZmFyaWNvbSxEQz1uZXQ/Y2VydGlm
aWNhdGVSZXZvY2F0aW9uTGlzdD9iYXNlP29iamVjdENsYXNzPWNSTERpc3RyaWJ1
dGlvblBvaW50hkdodHRwOi8vY3JsLnNhZmFyaWNvbS5jby5rZS9TYWZhcmljb20l
MjBJbnRlcm5hbCUyMElzc3VpbmclMjBDQSUyMDAyLmNybDCCAQkGCCsGAQUFBwEB
BIH8MIH5MIHJBggrBgEFBQcwAoaBvGxkYXA6Ly8vQ049U2FmYXJpY29tJTIwSW50
ZXJuYWwlMjBJc3N1aW5nJTIwQ0ElMjAwMixDTj1BSUEsQ049UHVibGljJTIwS2V5
JTIwU2VydmljZXMsQ049U2VydmljZXMsQ049Q29uZmlndXJhdGlvbixEQz1zYWZh
cmljb20sREM9bmV0P2NBQ2VydGlmaWNhdGU/YmFzZT9vYmplY3RDbGFzcz1jZXJ0
aWZpY2F0aW9uQXV0aG9yaXR5MCsGCCsGAQUFBzABhh9odHRwOi8vY3JsLnNhZmFy
aWNvbS5jby5rZS9vY3NwMAsGA1UdDwQEAwIFoDA9BgkrBgEEAYI3FQcEMDAuBiYr
BgEEAYI3FQiHz4xWhMLEA4XphTaE3tENhqCICGeGwcdsg7m5awIBZAIBDDAdBgNV
HSUEFjAUBggrBgEFBQcDAgYIKwYBBQUHAwEwJwYJKwYBBAGCNxUKBBowGDAKBggr
BgEFBQcDAjAKBggrBgEFBQcDATANBgkqhkiG9w0BAQsFAAOCAQEAC/hWx7KTwSYr
x2SOyyHNLTRmCnCJmqxA/Q+IzpW1mGtw4Sb/8jdsoWrDiYLxoKGkgkvmQmB2J3zU
ngzJIM2EeU921vbjLqX9sLWStZbNC2Udk5HEecdpe1AN/ltIoE09ntglUNINyCmf
zChs2maF0Rd/y5hGnMM9bX9ub0sqrkzL3ihfmv4vkXNxYR8k246ZZ8tjQEVsKehE
dqAmj8WYkYdWIHQlkKFP9ba0RJv7aBKb8/KP+qZ5hJip0I5Ey6JJ3wlEWRWUYUKh
gYoPHrJ92ToadnFCCpOlLKWc0xVxANofy6fqreOVboPO0qTAYpoXakmgeRNLUiar
0ah6M/q/KA==
-----END CERTIFICATE-----
"#,
            Environment::Sandbox => r#"-----BEGIN CERTIFICATE-----
MIIGKzCCBROgAwIBAgIQDL7NH8cxSdUpl0ihH0A1wTANBgkqhkiG9w0BAQsFADBN
MQswCQYDVQQGEwJVUzEVMBMGA1UEChMMRGlnaUNlcnQgSW5jMScwJQYDVQQDEx5E
aWdpQ2VydCBTSEEyIFNlY3VyZSBTZXJ2ZXIgQ0EwHhcNMTgwODI3MDAwMDAwWhcN
MTkwNDA0MTIwMDAwWjBuMQswCQYDVQQGEwJLRTEQMA4GA1UEBxMHTmFpcm9iaTEW
MBQGA1UEChMNU2FmYXJpY29tIFBMQzETMBEGA1UECxMKRGlnaXRhbCBJVDEgMB4G
A1UEAxMXc2FuZGJveC5zYWZhcmljb20uY28ua2UwggEiMA0GCSqGSIb3DQEBAQUA
A4IBDwAwggEKAoIBAQC78yeC/wLoZY6TJeqc4g/9eAKIpeCwEsjX09pD8ZxAGXqT
Oi7ssdIGJBPmJZNeEVyf8ocFhisCuLngJ9Z5e/AvH52PhrEFmVu2D03zSf4C+rhZ
ndEKP6G79pUAb/bemOliU9zM8xYYkpCRzPWUzk6zSDarg0ZDLw5FrtZj/VJ9YEDL
WGgAfwExEgSN3wjyUlJ2UwI3wqQXLka0VNFWoZxUH5j436gbSWRIL6NJUmrq8V8S
aTEPz3eJHj3NOToDu245c7VKdF/KExyZjRjD2p5I+Aip80TXzKlZj6DjMb3DlfXF
Hsnu0+1uJE701mvKX7BiscxKr8tCRphL63as4dqvAgMBAAGjggLkMIIC4DAfBgNV
HSMEGDAWgBQPgGEcgjFh1S8o541GOLQs4cbZ4jAdBgNVHQ4EFgQUzZmY7ZORLw9w
qRbAQN5m9lJ28qMwIgYDVR0RBBswGYIXc2FuZGJveC5zYWZhcmljb20uY28ua2Uw
DgYDVR0PAQH/BAQDAgWgMB0GA1UdJQQWMBQGCCsGAQUFBwMBBggrBgEFBQcDAjBr
BgNVHR8EZDBiMC+gLaArhilodHRwOi8vY3JsMy5kaWdpY2VydC5jb20vc3NjYS1z
aGEyLWc2LmNybDAvoC2gK4YpaHR0cDovL2NybDQuZGlnaWNlcnQuY29tL3NzY2Et
c2hhMi1nNi5jcmwwTAYDVR0gBEUwQzA3BglghkgBhv1sAQEwKjAoBggrBgEFBQcC
ARYcaHR0cHM6Ly93d3cuZGlnaWNlcnQuY29tL0NQUzAIBgZngQwBAgIwfAYIKwYB
BQUHAQEEcDBuMCQGCCsGAQUFBzABhhhodHRwOi8vb2NzcC5kaWdpY2VydC5jb20w
RgYIKwYBBQUHMAKGOmh0dHA6Ly9jYWNlcnRzLmRpZ2ljZXJ0LmNvbS9EaWdpQ2Vy
dFNIQTJTZWN1cmVTZXJ2ZXJDQS5jcnQwCQYDVR0TBAIwADCCAQUGCisGAQQB1nkC
BAIEgfYEgfMA8QB2AKS5CZC0GFgUh7sTosxncAo8NZgE+RvfuON3zQ7IDdwQAAAB
ZXs1FvEAAAQDAEcwRQIgBzVMkm7SNprjJ1GBqiXIc9rNzY+y7gt6s/O02oMkyFoC
IQDBuThGlpmUKpeZoHhK6HGwB4jDMIecmKaOcMS18R2jxwB3AId1v+dZfPiMQ5lf
vfNu/1aNR1Y2/0q1YMG06v9eoIMPAAABZXs1F8IAAAQDAEgwRgIhAIRq2XFiC+RS
uDCYq8ICJg0QafSV+e9BLpJnElEdaSjiAiEAyiiW4vxwv4cWcAXE6FAipctyUBs6
bE5QyaCnmNpoDiQwDQYJKoZIhvcNAQELBQADggEBAB0YoWve9Sxhb0PBS3Hc46Rf
a7H1jhHuwE+UyscSQsdJdk8uPAgDuKRZMvJPGEaCkNHm36NfcaXXFjPOl7LI1d1a
9zqSP0xeZBI6cF0x96WuQGrI9/WR2tfxjmaUSp8a/aJ6n+tZA28eJZNPrIaMm+6j
gh7AkKnqcf+g8F/MvCCVdNAiVMdz6UpCscf6BRPHNZ5ifvChGh7aUKjrVLLuF4Ls
HE05qm6HNyV5eTa6wvcbc4ewguN1UDZvPWetSyfBk10Wbpor4znQ4TJ3Y9uCvsJH
41ldblDvZZ2z4kB2UYQ7iBkPlJSxSOaFgW/GGDXq49sz/995xzhVITHxh2SdLkI=
-----END CERTIFICATE-----
 "#,
        }
    }
}