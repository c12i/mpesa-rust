This API generates the tokens for authenticating your API calls. This is the first API you will engage with within the set of APIs available because all the other APIs require authentication information from this API to work.

Safaricom API docs [reference](https://developer.safaricom.co.ke/APIs/Authorization)

Returns auth token as a `String` that is ttl-cached in memory for subsequent requests.

# Errors
Returns a `MpesaError` on failure