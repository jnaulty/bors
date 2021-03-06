use super::{
    DateTime, Label, Milestone, NodeId, Oid, ReactionSummary, Repository, State, Team, User,
};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct CommitRef {
    pub label: String,
    #[serde(rename = "ref")]
    pub git_ref: String,
    pub sha: Oid,
    pub user: User,
    pub repo: Option<Repository>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestState {
    Open,
    Closed,
    Merged,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PullRequest {
    pub url: String,
    pub id: u64,
    pub node_id: NodeId,
    pub html_url: String,
    pub diff_url: String,
    pub patch_url: String,
    pub issue_url: String,
    pub number: u64,
    pub state: State,
    pub locked: bool,
    pub title: String,
    pub user: User,
    pub body: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub closed_at: Option<DateTime>,
    pub merged_at: Option<DateTime>,
    pub merge_commit_sha: Option<Oid>,
    pub assignee: Option<User>,
    pub assignees: Vec<User>,
    pub requested_reviewers: Vec<User>,
    pub requested_teams: Vec<Team>,
    pub labels: Vec<Label>,
    pub milestone: Option<Milestone>,
    pub commits_url: String,
    pub review_comments_url: String,
    pub review_comment_url: String,
    pub comments_url: String,
    pub statuses_url: String,
    pub head: CommitRef,
    pub base: CommitRef,
    // pub _links: ???
    pub author_association: String,
    pub draft: Option<bool>,
    pub merged: Option<bool>,
    pub mergeable: Option<bool>,
    pub rebaseable: Option<bool>,
    pub mergeable_state: Option<String>,
    pub merged_by: Option<User>,
    pub comments: Option<u64>,
    pub review_comments: Option<u64>,
    pub maintainer_can_modify: Option<bool>,
    pub commits: Option<u64>,
    pub additions: Option<u64>,
    pub deletions: Option<u64>,
    pub changed_files: Option<u64>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewState {
    Approved,
    ChangesRequested,
    Commented,
    Dismissed,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Review {
    pub id: u64,
    pub node_id: NodeId,
    pub user: User,
    pub body: Option<String>,
    pub commit_id: Oid,
    pub submitted_at: DateTime,
    pub state: ReviewState,
    pub html_url: String,
    pub pull_request_url: String,
    // pub _links
    pub author_association: String, //TODO make a type
}

impl Review {
    pub fn body(&self) -> Option<&str> {
        self.body.as_ref().map(AsRef::as_ref)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ReviewComment {
    pub url: String,
    pub id: u64,
    pub node_id: NodeId,
    pub pull_request_review_id: u64,
    pub diff_hunk: String,
    pub path: String,
    pub position: Option<u64>,
    pub original_position: u64,
    pub commit_id: Oid,
    pub original_commit_id: Oid,
    pub in_reply_to_id: Option<u64>,
    pub user: User,
    pub body: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub html_url: String,
    pub pull_request_url: String,
    pub author_association: String,
    // pub _links
    pub start_line: Option<u64>,
    pub original_start_line: Option<u64>,
    pub start_side: Option<String>,
    pub line: Option<u64>,
    pub original_line: Option<u64>,
    pub side: Option<String>,
    pub reactions: Option<ReactionSummary>,
}

impl ReviewComment {
    pub fn body(&self) -> Option<&str> {
        self.body.as_ref().map(AsRef::as_ref)
    }
}

#[cfg(test)]
mod test {
    use super::PullRequest;

    #[test]
    fn pull_request() {
        const PR_JSON: &str = include_str!("../test-input/pr.json");
        let _pr: PullRequest = serde_json::from_str(PR_JSON).unwrap();
    }
}
