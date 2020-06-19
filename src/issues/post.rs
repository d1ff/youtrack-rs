//! Access the Issues portion of the YouTrack API
imports!();
use crate::client::PostQueryBuilder;

new_type!(
    Issues
    IssuesIssueId
    IssuesIssueIdVoters);

from!(
    @PostQueryBuilder
        -> Issues = "issues"
    @Issues
        => IssuesIssueId
    @IssuesIssueId
        -> IssuesIssueIdVoters = "voters");

impl_macro!(
    @Issues
        |
        |=> id -> IssuesIssueId = id_str
    @IssuesIssueId
        |=> voters -> IssuesIssueIdVoters
        |);

exec!(IssuesIssueIdVoters);
