//! The `Account` model.
//!
//! This module defines our document type along with its fields
//! and their mapping.
//! Field serialisation and mapping is all handled in the same place
//! so it's always in sync.

use elastic::types::prelude::{FieldType, KeywordFieldType, Text, DefaultTextMapping, TextMapping, Keyword,
                              DefaultKeywordMapping, DocumentType};

/// Our main model; an account in the bank.
#[derive(Debug, Serialize, Deserialize, ElasticType)]
pub struct Account {
    pub account_number: i32,
    pub balance: i32,
    pub firstname: FirstName,
    pub lastname: LastName,
    pub age: i8,
    pub gender: Gender,
    pub address: Address,
    pub employer: Employer,
    pub email: Email,
    pub city: City,
    pub state: State,
}

/// Get the indexed document type name.
pub fn name() -> &'static str {
    Account::name()
}

/// Get the strongly typed document mapping.
pub fn mapping() -> AccountMapping {
    Account::mapping()
}

// We're using type aliases to make the `Account` definition more ergonomic.

pub type Address = Text<DefaultTextMapping>;
pub type City = Keyword<DefaultKeywordMapping>;
pub type Employer = Keyword<DefaultKeywordMapping>;
pub type FirstName = Keyword<DefaultKeywordMapping>;
pub type LastName = Keyword<DefaultKeywordMapping>;
pub type State = Keyword<DefaultKeywordMapping>;

// You can implement your own field types as well as document types.
// The below `Gender` type is mapped as a `Keyword` in Elasticsearch,
// but is a strongly typed enum in Rust.
// This is done by implementing the `FieldType` trait using a `KeywordMapping`
// and a `KeywordFormat`.

#[derive(Debug, Serialize, Deserialize)]
pub enum Gender {
    #[serde(rename = "F")]
    Female,
    #[serde(rename = "M")]
    Male,
}

impl KeywordFieldType<DefaultKeywordMapping> for Gender {}

// The `Email` type uses a custom analyser so it has its own
// mapping type instead of using `DefaultKeywordMapping`.

pub type Email = Text<EmailMapping>;

#[derive(Debug, Default)]
pub struct EmailMapping;
impl TextMapping for EmailMapping {
    fn analyzer() -> Option<&'static str> {
        Some("email")
    }
}

#[cfg(test)]
mod tests {
    use serde_json;
    use elastic::types::prelude::Document;
    use super::{mapping, Account};

    #[test]
    fn deserialize() {
        let ser = json_str!({
            "account_number":1,
            "balance":39225,
            "firstname":"Amber",
            "lastname":"Duke",
            "age":32,
            "gender":"M",
            "address":"880 Holmes Lane",
            "employer":"Pyrami",
            "email":"amberduke@pyrami.com",
            "city":"Brogan",
            "state":"IL"
        });

        let de: Result<Account, _> = serde_json::from_str(&ser);

        assert!(de.is_ok());
    }

    #[test]
    fn serialise_mapping() {
        let ser = serde_json::to_string(&Document::from(mapping())).unwrap();

        let expected = json_str!({
            "properties":{
                "account_number":{
                    "type":"integer"
                },
                "balance":{
                    "type":"integer"
                },
                "firstname":{
                    "type":"keyword"
                },
                "lastname":{
                    "type":"keyword"
                },
                "age":{
                    "type":"byte"
                },
                "gender":{
                    "type":"keyword"
                },
                "address":{
                    "type":"text"
                },
                "employer":{
                    "type":"keyword"
                },
                "email":{
                    "type":"text",
                    "analyzer":"email"
                },
                "city":{
                    "type":"keyword"
                },
                "state":{
                    "type":"keyword"
                }
            }
        });

        assert_eq!(expected, ser);
    }
}
