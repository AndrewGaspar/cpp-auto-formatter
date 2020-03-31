use serde::Deserialize;

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct GitHubRepository {
    pub id: usize,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    pub owner: GitHubUser,
    pub private: bool,
    pub html_url: String,
    pub description: String,
    pub fork: bool,
    pub url: String,
    pub archive_url: String,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub deployments_url: String,
    pub downloads_url: String,
    pub events_url: String,
    pub forks_url: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub git_url: String,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub languages_url: String,
    pub merges_url: String,
    pub milestones_url: String,
    pub notifications_url: String,
    pub pulls_url: String,
    pub releases_url: String,
    pub ssh_url: String,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub trees_url: String,
    pub clone_url: String,
    pub mirror_url: String,
    pub hooks_url: String,
    pub svn_url: String,
    pub homepage: String,
    pub language: Option<String>,
    pub forks_count: usize,
    pub stargazers_count: usize,
    pub watchers_count: usize,
    pub size: usize,
    pub default_branch: String,
    pub open_issues_count: usize,
    pub is_template: bool,
    pub topics: Vec<String>,
    pub has_issues: bool,
    pub has_projects: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub has_downloads: bool,
    pub archived: bool,
    pub disabled: bool,
    pub visibility: String,
    pub pushed_at: String,
    pub created_at: String,
    pub updated_at: String,
    pub permissions: GitHubRepositoryPermissions,
    pub allow_rebase_merge: bool,
    pub template_repository: Option<String>,
    pub temp_clone_token: String,
    pub allow_squash_merge: bool,
    pub allow_merge_commit: bool,
    pub subscribers_count: usize,
    pub network_count: usize,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct GitHubRepositoryPermissions {
    admin: bool,
    push: bool,
    pull: bool,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct GitHubIssueCommentEvent {
    pub action: String,
    pub comment: GitHubIssueComment,
    pub issue: GitHubIssue,
    pub repository: GitHubRepository,
    pub sender: GitHubUser,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct GitHubIssueComment {
    pub url: String,
    pub html_url: String,
    pub issue_url: String,
    pub id: usize,
    pub node_id: String,
    pub user: GitHubUser,
    pub created_at: String,
    pub updated_at: String,
    pub author_association: String,
    pub body: String,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct GitHubIssue {
    pub id: usize,
    pub node_id: String,
    pub url: String,
    pub repository_url: String,
    pub labels_url: String,
    pub comments_url: String,
    pub events_url: String,
    pub html_url: String,
    pub number: usize,
    pub state: String,
    pub title: String,
    pub body: String,
    pub author_association: String,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub comments: usize,
    pub pull_request: Option<GitHubIssuePullRequest>,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct GitHubIssuePullRequest {
    pub url: String,
    pub html_url: String,
    pub diff_url: String,
    pub patch_url: String,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct GitHubUser {
    pub login: String,
    pub id: usize,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    pub r#type: String,
    pub site_admin: bool,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct GitHubPullRequest {
    pub url: String,
    pub id: usize,
    pub node_id: String,
    pub html_url: String,
    pub diff_url: String,
    pub patch_url: String,
    pub issue_url: String,
    pub commits_url: String,
    pub review_comments_url: String,
    pub review_comment_url: String,
    pub comments_url: String,
    pub statuses_url: String,
    pub number: usize,
    pub state: String,
    pub locked: bool,
    pub title: String,
    pub user: GitHubUser,
    pub body: String,
    pub active_lock_reason: String,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: String,
    pub merged_at: String,
    pub merge_commit_sha: String,
    pub assignee: GitHubUser,
    pub assignees: Vec<GitHubUser>,
    pub requested_reviewers: Vec<GitHubUser>,
    pub head: GitHubPullRequestBranch,
    pub base: GitHubPullRequestBranch,
    pub author_association: String,
    pub draft: bool,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct GitHubPullRequestBranch {
    pub label: String,
    pub r#ref: String,
    pub sha: String,
    pub user: GitHubUser,
}
