#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::redundant_clone)]
pub mod models;
#[derive(Clone)]
pub struct Client {
    endpoint: String,
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    scopes: Vec<String>,
    pipeline: azure_core::Pipeline,
}
#[derive(Clone)]
pub struct ClientBuilder {
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    endpoint: Option<String>,
    scopes: Option<Vec<String>>,
    options: azure_core::ClientOptions,
}
pub const DEFAULT_ENDPOINT: &str = azure_core::resource_manager_endpoint::AZURE_PUBLIC_CLOUD;
impl ClientBuilder {
    #[doc = "Create a new instance of `ClientBuilder`."]
    #[must_use]
    pub fn new(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> Self {
        Self {
            credential,
            endpoint: None,
            scopes: None,
            options: azure_core::ClientOptions::default(),
        }
    }
    #[doc = "Set the endpoint."]
    #[must_use]
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }
    #[doc = "Set the scopes."]
    #[must_use]
    pub fn scopes(mut self, scopes: &[&str]) -> Self {
        self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
        self
    }
    #[doc = "Set the retry options."]
    #[must_use]
    pub fn retry(mut self, retry: impl Into<azure_core::RetryOptions>) -> Self {
        self.options = self.options.retry(retry);
        self
    }
    #[doc = "Set the transport options."]
    #[must_use]
    pub fn transport(mut self, transport: impl Into<azure_core::TransportOptions>) -> Self {
        self.options = self.options.transport(transport);
        self
    }
    #[doc = "Convert the builder into a `Client` instance."]
    #[must_use]
    pub fn build(self) -> Client {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
        let scopes = self.scopes.unwrap_or_else(|| vec![format!("{}/", endpoint)]);
        Client::new(endpoint, self.credential, scopes, self.options)
    }
}
impl Client {
    pub(crate) fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }
    pub(crate) fn token_credential(&self) -> &dyn azure_core::auth::TokenCredential {
        self.credential.as_ref()
    }
    pub(crate) fn scopes(&self) -> Vec<&str> {
        self.scopes.iter().map(String::as_str).collect()
    }
    pub(crate) async fn send(&self, request: &mut azure_core::Request) -> azure_core::Result<azure_core::Response> {
        let mut context = azure_core::Context::default();
        self.pipeline.send(&mut context, request).await
    }
    #[doc = "Create a new `ClientBuilder`."]
    #[must_use]
    pub fn builder(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> ClientBuilder {
        ClientBuilder::new(credential)
    }
    #[doc = "Create a new `Client`."]
    #[must_use]
    pub fn new(
        endpoint: impl Into<String>,
        credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
        scopes: Vec<String>,
        options: azure_core::ClientOptions,
    ) -> Self {
        let endpoint = endpoint.into();
        let pipeline = azure_core::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options,
            Vec::new(),
            Vec::new(),
        );
        Self {
            endpoint,
            credential,
            scopes,
            pipeline,
        }
    }
    pub fn email_client(&self) -> email::Client {
        email::Client(self.clone())
    }
}
pub mod email {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the status of a message sent previously."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `message_id`: System generated message id (GUID) returned from a previous call to send email"]
        pub fn get_send_status(&self, message_id: impl Into<String>) -> get_send_status::RequestBuilder {
            get_send_status::RequestBuilder {
                client: self.0.clone(),
                message_id: message_id.into(),
            }
        }
        #[doc = "Queues an email message to be sent to one or more recipients"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `repeatability_request_id`: If specified, the client directs that the request is repeatable; that is, that the client can make the request multiple times with the same Repeatability-Request-Id and get back an appropriate response without the server executing the request multiple times. The value of the Repeatability-Request-Id is an opaque string representing a client-generated, globally unique for all time, identifier for the request. It is recommended to use version 4 (random) UUIDs."]
        #[doc = "* `repeatability_first_sent`: Must be sent by clients to specify that a request is repeatable. Repeatability-First-Sent is used to specify the date and time at which the request was first created in the IMF-fix date form of HTTP-date as defined in RFC7231. eg- Tue, 26 Mar 2019 16:06:51 GMT"]
        #[doc = "* `email_message`: Message payload for sending an email"]
        pub fn send(
            &self,
            repeatability_request_id: impl Into<String>,
            repeatability_first_sent: impl Into<String>,
            email_message: impl Into<models::EmailMessage>,
        ) -> send::RequestBuilder {
            send::RequestBuilder {
                client: self.0.clone(),
                repeatability_request_id: repeatability_request_id.into(),
                repeatability_first_sent: repeatability_first_sent.into(),
                email_message: email_message.into(),
            }
        }
    }
    pub mod get_send_status {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SendStatusResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SendStatusResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "Amount of time client should wait before retrying the request, specified in seconds "]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) message_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/emails/{}/status", this.client.endpoint(), &this.message_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::SendStatusResult>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod send {
        use super::models;
        use httpdate::fmt_http_date;
        use sha2::{Sha256, Digest};
        use base64;
        use hmac::{Hmac,Mac};
        use std::time::SystemTime;
        #[derive(Debug)]
        pub struct Response(pub azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) repeatability_request_id: String,
            pub(crate) repeatability_first_sent: String,
            pub(crate) email_message: models::EmailMessage,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/emails:send", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01-preview");
                        req.insert_header("repeatability-request-id", &this.repeatability_request_id);
                        req.insert_header("repeatability-first-sent", &this.repeatability_first_sent);
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.email_message)?;

                        // Changed Bearer Auth to HMAC-SHA256 === Quick and dirty inline impl
                        
                        // Step 1, generate HTTP Date
                        let http_date = fmt_http_date(SystemTime::now());
                        req.insert_header("x-ms-date",http_date.clone());

                        // Step 2, hash the body content and add it to header
                        let mut hasher = Sha256::new();
                        hasher.update(req_body.clone());
                        let hash = hasher.finalize();
                        let base64_hash = base64::encode(&hash);
                        req.insert_header("x-ms-content-sha256",base64_hash.clone());

                        // Step 3, prepare the HMAC string
                        let mut host = &this.client.endpoint()[8..]; // Remove https://
                        host = &host[..host.len()-1]; // Remove last '/'

                        let string_to_sign = format!("POST\n/emails:send?api-version=2021-10-01-preview\n{};{};{}",http_date, host, base64_hash);

                        // Step 4, Sign the HMAC with the provided ACS Key/Secret
                        let hmac_key = base64::decode(token_response.token.secret()).unwrap(); // Todo throw error if invalid Key
                        let mut hmac = Hmac::<Sha256>::new_from_slice(&hmac_key).unwrap();
                        hmac.update(string_to_sign.as_bytes());
                        let signature = base64::encode(hmac.finalize().into_bytes());

                        // Step 5, Insert Signature as Auth header
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("HMAC-SHA256 SignedHeaders=x-ms-date;host;x-ms-content-sha256&Signature={}", signature),
                        );
                        req.set_body(req_body);
 
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
