/*
Creates a batch job and task using the data plane APIs

cargo run --package azure_svc_batch --example create_task
*/
use azure_svc_communicationemail:: {
  models:: { EmailRecipients, EmailAddress, EmailContent, EmailMessage},
  email::send:: {Response as EmailResponse}
};
use azure_core::auth:: {AccessToken, TokenCredential, TokenResponse};
use std::sync::Arc;
use time::OffsetDateTime;
use uuid::Uuid;
use std::time::SystemTime;
use httpdate::fmt_http_date;

// Boilerplate code to pass a simple ACS Key to the HTTP request as auth token
struct AcsKey {
    key : String,
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for AcsKey {
    async fn get_token(&self, _resource: &str) -> azure_core::Result<TokenResponse> {
        Ok(TokenResponse::new(AccessToken::new(self.key.clone()),
        OffsetDateTime::UNIX_EPOCH)) 
    }
}
// Start of MAIN

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint= std::env::args().nth(1).expect("please specify ACS endpoint");
    let credstr = std::env::args().nth(2).expect("please specify ACS Credential");

    let credential = Arc::new(AcsKey{key:credstr});
    println!("creating email client");
    let client = azure_svc_communicationemail::Client::builder(credential)
        .endpoint(endpoint)
        .build();

    let recipients = EmailRecipients::new([EmailAddress{email:"vincentpgerard@gmail.com".to_string(),
                                                        display_name:Some("Vincent Gerard".to_string())}].to_vec());
    let content = EmailContent{subject:"Email from Rust using ACS".to_string(),
                               plain_text:Some("this is a test email".to_string()),
                               html:None};
    let email = EmailMessage::new("DoNotReply@vgerard.net".to_string(), content, recipients);
    
    println!("created email object");

    let uuid = Uuid::new_v4();
    let http_date = fmt_http_date(SystemTime::now());
    println!("RepeatabilityId: {}, Http_date: {}",uuid, http_date);

    let to_be_sent = client.email_client().send(uuid.to_string(), http_date,email);

    let response = to_be_sent.send().await;

    match response {
         Err(e) => { println!("Error returned from Send Email API: {}",e) }
         Ok(EmailResponse(s)) =>  { 
             let (status,headers,_body) = s.deconstruct();
             println!("Success, HTTP Reply status: {}",status);
             println!("Returned headers: {headers:?}");
         }
    }

    Ok(())
}
