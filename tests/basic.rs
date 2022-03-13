use mockito::mock;

use migadu::{Credentials, Mailbox, Migadu};

fn mock_endpoint() -> String {
    mockito::server_url()
}

#[tokio::test]
async fn test_api_mock() {
    let _m = mock("GET", "/mockdomain.com/mailboxes")
        .with_status(200)
        .with_header("content-type", "text/json")
        .with_body(
            r#"{
            "mailboxes": [
              {
                "address": "admin@mockdomain.com",
                "domain_name": "mockdomain.com",
              },
              {
                "address": "info@mockdomain.com",
                "domain_name": "mockdomain.com",
              }
            ]
          }"#,
        )
        .create();

    let migadu = Migadu {
        api: migadu::API::Mailboxes {
            data: Mailbox {
                ..Default::default()
            },
            extra: None,
            action: migadu::Action::INDEX,
        },
        credentials: Credentials {
            user: "mockuser@email.com".to_string(),
            pass: "123344-secure-api-key".to_string(),
            domain: "mockdomain.com".to_string(),
        },
    };

    let result = migadu.run(mock_endpoint()).await;
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap().text().await.unwrap(),
        r#"{
            "mailboxes": [
              {
                "address": "admin@mockdomain.com",
                "domain_name": "mockdomain.com",
              },
              {
                "address": "info@mockdomain.com",
                "domain_name": "mockdomain.com",
              }
            ]
          }"#
        .to_string()
    );
}
