use serde::Deserialize;
use std::time::SystemTime;

trait WebhookPayload {}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Repository {
    id: i32,
    full_name: String,
    name: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct GithubUser {
    id: i32,
    login: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")] // Converts enum variants to lowercase
pub enum StarAction {
    Created,
    Deleted,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct StarPayload {
    action: StarAction,
    repository: Repository,
    sender: GithubUser,
    starred_at: Option<SystemTime>,
}

impl WebhookPayload for StarPayload {}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Forkee {
    description: String,
    forks_count: i32,
    full_name: String,
    id: i32,
    name: String,
    private: bool,
    stargazers_count: i32,
    url: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ForkPayload {
    forkee: Forkee,
    repository: Repository,
    sender: GithubUser,
}

impl WebhookPayload for ForkPayload {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::payloads::StarAction::Created;

    #[test]
    fn deserialize_star_payload() {
        let json = r#"
        {
            "action": "created",
            "repository": {
                "id": 123456,
                "name": "motivate-me",
                "full_name": "manuelarte/motivate-me"
            },
            "sender": {
                "id": 1,
                "login": "octocat"
            }
        }
        "#;
        let actual: StarPayload = serde_json::from_str(json).unwrap();
        let repository = Repository {
            id: 123456,
            full_name: "manuelarte/motivate-me".to_owned(),
            name: "motivate-me".to_owned(),
        };
        let sender = GithubUser {
            id: 1,
            login: "octocat".to_owned(),
        };
        let expected = StarPayload {
            action: Created,
            repository,
            sender,
            starred_at: None,
        };
        assert_eq!(actual, expected)
    }

    #[test]
    fn deserialize_fork_payload() {
        let json = r#"
        {
          "forkee": {
            "id": 654321,
            "name": "motivate-me",
            "full_name": "octocat/motivate-me",
            "description": "motivate-me repo",
            "forks_count": 1,
            "private": false,
            "stargazers_count": 10,
            "url": "https://github.com/manuelarte/motivate-me"
          },
          "repository": {
            "id": 123456,
            "name": "motivate-me",
            "full_name": "manuelarte/motivate-me"
          },
          "sender": {
            "login": "octocat",
            "id": 1
          }
        }
        "#;
        let actual: ForkPayload = serde_json::from_str(json).unwrap();
        let forkee: Forkee = Forkee {
            description: "motivate-me repo".to_owned(),
            forks_count: 1,
            full_name: "octocat/motivate-me".to_owned(),
            id: 654321,
            name: "motivate-me".to_owned(),
            private: false,
            stargazers_count: 10,
            url: "https://github.com/manuelarte/motivate-me".to_owned(),
        };
        let repository = Repository {
            id: 123456,
            full_name: "manuelarte/motivate-me".to_owned(),
            name: "motivate-me".to_owned(),
        };
        let sender = GithubUser {
            id: 1,
            login: "octocat".to_owned(),
        };
        let expected = ForkPayload {
            forkee,
            repository,
            sender,
        };
        assert_eq!(actual, expected)
    }
}
