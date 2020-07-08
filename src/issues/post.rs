//! Access the Issues portion of the YouTrack API
imports!();
use crate::client::PostQueryBuilder;

new_type!(
    Issues
    IssuesFields
    IssuesIssueId
    IssuesIssueIdVoters);

from!(
    @PostQueryBuilder
        -> Issues = "issues"
    @Issues
        => IssuesIssueId
    @Issues
        ?> IssuesFields = "fields"
    @IssuesIssueId
        -> IssuesIssueIdVoters = "voters");

impl_macro!(
    @Issues
        |
        |=> id -> IssuesIssueId = id_str
        |?> fields -> IssuesFields = fields
    @IssuesIssueId
        |=> voters -> IssuesIssueIdVoters
        |);

exec!(IssuesIssueIdVoters);
exec!(IssuesFields);
