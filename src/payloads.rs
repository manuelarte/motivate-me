use chrono::{DateTime, Utc};
use serde::Deserialize;

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
    starred_at: Option<DateTime<Utc>>,
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
    use serde_json::Error;

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

    #[test]
    fn real_star_example_test() {
        let json = r#"
        {
            "action": "created",
            "starred_at": "2025-06-22T09:56:51Z",
            "repository": {
                "id": 1005506273,
                "node_id": "R_kgDOO-7O4Q",
                "name": "motivate-me",
                "full_name": "manuelarte/motivate-me",
                "private": false,
                "owner": {
                    "login": "manuelarte",
                    "id": 5348899,
                    "node_id": "MDQ6VXNlcjUzNDg4OTk=",
                    "avatar_url": "https://avatars.githubusercontent.com/u/5348899?v=4",
                    "gravatar_id": "",
                    "url": "https://api.github.com/users/manuelarte",
                    "html_url": "https://github.com/manuelarte",
                    "followers_url": "https://api.github.com/users/manuelarte/followers",
                    "following_url": "https://api.github.com/users/manuelarte/following{/other_user}",
                    "gists_url": "https://api.github.com/users/manuelarte/gists{/gist_id}",
                    "starred_url": "https://api.github.com/users/manuelarte/starred{/owner}{/repo}",
                    "subscriptions_url": "https://api.github.com/users/manuelarte/subscriptions",
                    "organizations_url": "https://api.github.com/users/manuelarte/orgs",
                    "repos_url": "https://api.github.com/users/manuelarte/repos",
                    "events_url": "https://api.github.com/users/manuelarte/events{/privacy}",
                    "received_events_url": "https://api.github.com/users/manuelarte/received_events",
                    "type": "User",
                    "user_view_type": "public",
                    "site_admin": false
                },
                "html_url": "https://github.com/manuelarte/motivate-me",
                "description": "A way to motivate me to continue doing code development!",
                "fork": false,
                "url": "https://api.github.com/repos/manuelarte/motivate-me",
                "forks_url": "https://api.github.com/repos/manuelarte/motivate-me/forks",
                "keys_url": "https://api.github.com/repos/manuelarte/motivate-me/keys{/key_id}",
                "collaborators_url": "https://api.github.com/repos/manuelarte/motivate-me/collaborators{/collaborator}",
                "teams_url": "https://api.github.com/repos/manuelarte/motivate-me/teams",
                "hooks_url": "https://api.github.com/repos/manuelarte/motivate-me/hooks",
                "issue_events_url": "https://api.github.com/repos/manuelarte/motivate-me/issues/events{/number}",
                "events_url": "https://api.github.com/repos/manuelarte/motivate-me/events",
                "assignees_url": "https://api.github.com/repos/manuelarte/motivate-me/assignees{/user}",
                "branches_url": "https://api.github.com/repos/manuelarte/motivate-me/branches{/branch}",
                "tags_url": "https://api.github.com/repos/manuelarte/motivate-me/tags",
                "blobs_url": "https://api.github.com/repos/manuelarte/motivate-me/git/blobs{/sha}",
                "git_tags_url": "https://api.github.com/repos/manuelarte/motivate-me/git/tags{/sha}",
                "git_refs_url": "https://api.github.com/repos/manuelarte/motivate-me/git/refs{/sha}",
                "trees_url": "https://api.github.com/repos/manuelarte/motivate-me/git/trees{/sha}",
                "statuses_url": "https://api.github.com/repos/manuelarte/motivate-me/statuses/{sha}",
                "languages_url": "https://api.github.com/repos/manuelarte/motivate-me/languages",
                "stargazers_url": "https://api.github.com/repos/manuelarte/motivate-me/stargazers",
                "contributors_url": "https://api.github.com/repos/manuelarte/motivate-me/contributors",
                "subscribers_url": "https://api.github.com/repos/manuelarte/motivate-me/subscribers",
                "subscription_url": "https://api.github.com/repos/manuelarte/motivate-me/subscription",
                "commits_url": "https://api.github.com/repos/manuelarte/motivate-me/commits{/sha}",
                "git_commits_url": "https://api.github.com/repos/manuelarte/motivate-me/git/commits{/sha}",
                "comments_url": "https://api.github.com/repos/manuelarte/motivate-me/comments{/number}",
                "issue_comment_url": "https://api.github.com/repos/manuelarte/motivate-me/issues/comments{/number}",
                "contents_url": "https://api.github.com/repos/manuelarte/motivate-me/contents/{+path}",
                "compare_url": "https://api.github.com/repos/manuelarte/motivate-me/compare/{base}...{head}",
                "merges_url": "https://api.github.com/repos/manuelarte/motivate-me/merges",
                "archive_url": "https://api.github.com/repos/manuelarte/motivate-me/{archive_format}{/ref}",
                "downloads_url": "https://api.github.com/repos/manuelarte/motivate-me/downloads",
                "issues_url": "https://api.github.com/repos/manuelarte/motivate-me/issues{/number}",
                "pulls_url": "https://api.github.com/repos/manuelarte/motivate-me/pulls{/number}",
                "milestones_url": "https://api.github.com/repos/manuelarte/motivate-me/milestones{/number}",
                "notifications_url": "https://api.github.com/repos/manuelarte/motivate-me/notifications{?since,all,participating}",
                "labels_url": "https://api.github.com/repos/manuelarte/motivate-me/labels{/name}",
                "releases_url": "https://api.github.com/repos/manuelarte/motivate-me/releases{/id}",
                "deployments_url": "https://api.github.com/repos/manuelarte/motivate-me/deployments",
                "created_at": "2025-06-20T10:33:30Z",
                "updated_at": "2025-06-22T09:56:52Z",
                "pushed_at": "2025-06-22T05:25:56Z",
                "git_url": "git://github.com/manuelarte/motivate-me.git",
                "ssh_url": "git@github.com:manuelarte/motivate-me.git",
                "clone_url": "https://github.com/manuelarte/motivate-me.git",
                "svn_url": "https://github.com/manuelarte/motivate-me",
                "homepage": null,
                "size": 14,
                "stargazers_count": 1,
                "watchers_count": 1,
                "language": "Shell",
                "has_issues": true,
                "has_projects": true,
                "has_downloads": true,
                "has_wiki": false,
                "has_pages": false,
                "has_discussions": false,
                "forks_count": 0,
                "mirror_url": null,
                "archived": false,
                "disabled": false,
                "open_issues_count": 0,
                "license": null,
                "allow_forking": true,
                "is_template": false,
                "web_commit_signoff_required": false,
                "topics": [],
                "visibility": "private",
                "forks": 0,
                "open_issues": 0,
                "watchers": 1,
                "default_branch": "main"
            },
            "sender": {
                "login": "manuelarte",
                "id": 5348899,
                "node_id": "MDQ6VXNlcjUzNDg4OTk=",
                "avatar_url": "https://avatars.githubusercontent.com/u/5348899?v=4",
                "gravatar_id": "",
                "url": "https://api.github.com/users/manuelarte",
                "html_url": "https://github.com/manuelarte",
                "followers_url": "https://api.github.com/users/manuelarte/followers",
                "following_url": "https://api.github.com/users/manuelarte/following{/other_user}",
                "gists_url": "https://api.github.com/users/manuelarte/gists{/gist_id}",
                "starred_url": "https://api.github.com/users/manuelarte/starred{/owner}{/repo}",
                "subscriptions_url": "https://api.github.com/users/manuelarte/subscriptions",
                "organizations_url": "https://api.github.com/users/manuelarte/orgs",
                "repos_url": "https://api.github.com/users/manuelarte/repos",
                "events_url": "https://api.github.com/users/manuelarte/events{/privacy}",
                "received_events_url": "https://api.github.com/users/manuelarte/received_events",
                "type": "User",
                "user_view_type": "public",
                "site_admin": false
            }
        }
        "#;
        let actual: Result<StarPayload, Error> = serde_json::from_str(json);
        if let Err(e) = &actual {
            println!("{}", e)
        }
        assert!(actual.is_ok())
    }
}
