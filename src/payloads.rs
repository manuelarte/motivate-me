use std::time::SystemTime;
use serde::Deserialize;

trait WebhookPayload {}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Repository {
    id: i32,
    full_name: String,
    name: String,
}

impl Repository {
    fn new(id: i32, full_name: String, name: String) -> Repository {
        Repository{
            id,
            full_name,
            name,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct GithubUser {
    id: i32,
    login: String
}

impl GithubUser {
    fn new(id: i32, login: String) -> GithubUser {
        GithubUser{
            id,
            login
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")] // Converts enum variants to lowercase
pub enum StarAction {
    Created, Deleted
}


#[derive(Debug, Deserialize, PartialEq)]
pub struct StarPayload {
    action: StarAction,
    repository: Repository,
    sender: GithubUser,
    starred_at: Option<SystemTime>
}

impl StarPayload {
    fn new(action: StarAction, repository: Repository, sender: GithubUser, starred_at: Option<SystemTime>) -> StarPayload {
        StarPayload{
            action,
            repository,
            sender,
            starred_at,
        }
    }
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

impl Forkee {
    fn new(description: String, forks_count: i32, full_name: String, id: i32, name: String, private: bool,
           stargazers_count: i32, url: String) -> Forkee {
        Forkee{
            description,
            forks_count,
            full_name,
            id,
            name,
            private,
            stargazers_count,
            url
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ForkPayload {
    forkee: Forkee,
    repository: Repository,
    sender: GithubUser
}

impl WebhookPayload for ForkPayload {}

impl ForkPayload {
    fn new(forkee: Forkee, repository: Repository, sender: GithubUser) -> ForkPayload {
        ForkPayload{
            forkee,
            repository,
            sender
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::payloads::StarAction::Created;
    use super::*;

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
        let repository = Repository::new(123456, "manuelarte/motivate-me".to_owned(), "motivate-me".to_owned());
        let sender = GithubUser::new(1, "octocat".to_owned());
        let expected = StarPayload::new(Created, repository, sender, None);
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
        let forkee: Forkee = Forkee::new("motivate-me repo".to_owned(), 1,
                                         "octocat/motivate-me".to_owned(), 654321,
                                         "motivate-me".to_owned(), false, 10,
                                         "https://github.com/manuelarte/motivate-me".to_owned());
        let repository = Repository::new(123456, "manuelarte/motivate-me".to_owned(), "motivate-me".to_owned());
        let sender = GithubUser::new(1, "octocat".to_owned());
        let expected = ForkPayload::new(forkee, repository, sender);
        assert_eq!(actual, expected)
    }
}
