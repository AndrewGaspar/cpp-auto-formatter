use serde::{Deserialize, Serialize};

// Events

#[derive(Deserialize, Debug)]
pub struct GitHubIssueCommentEvent {
    pub action: String,
    pub comment: GitHubIssueComment,
    pub issue: GitHubIssue,
    pub repository: GitHubEventRepository,
}

#[derive(Deserialize, Debug)]
pub struct GitHubPushEvent {
    pub r#ref: String,
    pub repository: GitHubEventRepository,
}

#[derive(Deserialize, Debug)]
pub struct GitHubEventRepository {
    pub full_name: String,
}

#[derive(Deserialize, Debug)]
pub struct GitHubIssueComment {
    pub body: String,
}

#[derive(Deserialize, Debug)]
pub struct GitHubIssue {
    pub pull_request: Option<GitHubIssuePullRequest>,
}

#[derive(Deserialize, Debug)]
pub struct GitHubIssuePullRequest {
    pub url: String,
}

// API

#[derive(Deserialize, Debug)]
pub struct GitHubPullRequest {
    pub comments_url: String,
    pub head: GitHubPullRequestBranch,
}

#[derive(Deserialize, Debug)]
pub struct GitHubPullRequestBranch {
    pub r#ref: String,
    pub repo: GitHubRepository,
}

#[derive(Deserialize, Debug)]
pub struct GitHubRepository {
    pub full_name: String,
}

/// https://developer.github.com/v3/issues/comments/#create-a-comment
#[derive(Serialize, Debug)]
pub struct GitHubIssueCreate {
    pub body: String,
}
