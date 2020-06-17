//! Access the Issues portion of the YouTrack API
imports!();
use crate::client::GetQueryBuilder;

new_type!(
    Issues
    IssuesQuery
    IssuesQueryTop
    IssuesQueryTopSkip
    IssuesQueryTopSkipFields
    IssuesIssueId
    IssuesIssueIdLinks
    IssuesIssueIdAttachments
    IssuesIssueIdComments
    IssuesIssueIdTimeTracking
    IssuesIssueIdProject
    IssuesIssueIdCustomFields
    IssuesIssueIdActivities
    IssuesIssueIdActivitiesPage);

from!(
    @GetQueryBuilder
        -> Issues = "issues"
    @Issues
        => IssuesIssueId
    @Issues
        ?> IssuesQuery = "query"
    @IssuesQuery
        ?> IssuesQueryTop = "$top"
    @IssuesQueryTop
        ?> IssuesQueryTopSkip = "$skip"
    @IssuesQueryTopSkip
        ?> IssuesQueryTopSkipFields = "fields"
    @IssuesIssueId
        -> IssuesIssueIdLinks = "links"
        -> IssuesIssueIdAttachments = "attachments"
        -> IssuesIssueIdComments = "comments"
        -> IssuesIssueIdTimeTracking = "timeTracking"
        -> IssuesIssueIdProject = "project"
        -> IssuesIssueIdCustomFields = "customFields"
        -> IssuesIssueIdActivities = "activities"
        -> IssuesIssueIdActivitiesPage = "activitiesPage"
);

impl_macro!(
    @Issues
        |
        |=> id -> IssuesIssueId = id_str
        |?> query -> IssuesQuery = query_str
    @IssuesQuery
        |
        |?> top -> IssuesQueryTop = top
    @IssuesQueryTop
        |
        |?> skip -> IssuesQueryTopSkip = skip
    @IssuesQueryTopSkip
        |
        |?> fields -> IssuesQueryTopSkipFields = fields
    @IssuesIssueId
        |=> links -> IssuesIssueIdLinks
        |=> attachments -> IssuesIssueIdAttachments
        |=> comments -> IssuesIssueIdComments
        |=> time_tracking -> IssuesIssueIdTimeTracking
        |=> project -> IssuesIssueIdProject
        |=> custom_fields -> IssuesIssueIdCustomFields
        |=> activities -> IssuesIssueIdActivities
        |=> activities_page -> IssuesIssueIdActivitiesPage
        |);

exec!(IssuesQueryTopSkipFields);
/*exec!(IssuesIssueIdFields);*/
//exec!(IssuesIssueIdLinksFields);
//exec!(IssuesIssueIdAttachmentsFields);
//exec!(IssuesIssueIdCommentsFields);
//exec!(IssuesIssueIdTimeTrackingFields);
//exec!(IssuesIssueIdProjectFields);
//exec!(IssuesIssueIdCustomFieldsFields);
//exec!(IssuesIssueIdActivitiesFields);
/*exec!(IssuesIssueIdActivitiesPageFields);*/
