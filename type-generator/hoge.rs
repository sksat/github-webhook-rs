// BranchProtectionRuleCreatedEvent
// ts type alias
pub enum Schema {
  BranchProtectionRuleEvent(BranchProtectionRuleEvent),
  CheckRunEvent(CheckRunEvent),
  CheckSuiteEvent(CheckSuiteEvent),
  CodeScanningAlertEvent(CodeScanningAlertEvent),
  CommitCommentEvent(CommitCommentEvent),
  CreateEvent(CreateEvent),
  DeleteEvent(DeleteEvent),
  DependabotAlertEvent(DependabotAlertEvent),
  DeployKeyEvent(DeployKeyEvent),
  DeploymentEvent(DeploymentEvent),
  DeploymentStatusEvent(DeploymentStatusEvent),
  DiscussionEvent(DiscussionEvent),
  DiscussionCommentEvent(DiscussionCommentEvent),
  ForkEvent(ForkEvent),
  GithubAppAuthorizationEvent(GithubAppAuthorizationEvent),
  GollumEvent(GollumEvent),
  InstallationEvent(InstallationEvent),
  InstallationRepositoriesEvent(InstallationRepositoriesEvent),
  IssueCommentEvent(IssueCommentEvent),
  IssuesEvent(IssuesEvent),
  LabelEvent(LabelEvent),
  MarketplacePurchaseEvent(MarketplacePurchaseEvent),
  MemberEvent(MemberEvent),
  MembershipEvent(MembershipEvent),
  MergeGroupEvent(MergeGroupEvent),
  MetaEvent(MetaEvent),
  MilestoneEvent(MilestoneEvent),
  OrgBlockEvent(OrgBlockEvent),
  OrganizationEvent(OrganizationEvent),
  PackageEvent(PackageEvent),
  PageBuildEvent(PageBuildEvent),
  PingEvent(PingEvent),
  ProjectEvent(ProjectEvent),
  ProjectCardEvent(ProjectCardEvent),
  ProjectColumnEvent(ProjectColumnEvent),
  ProjectsV2ItemEvent(ProjectsV2ItemEvent),
  PublicEvent(PublicEvent),
  PullRequestEvent(PullRequestEvent),
  PullRequestReviewEvent(PullRequestReviewEvent),
  PullRequestReviewCommentEvent(PullRequestReviewCommentEvent),
  PullRequestReviewThreadEvent(PullRequestReviewThreadEvent),
  PushEvent(PushEvent),
  RegistryPackageEvent(RegistryPackageEvent),
  ReleaseEvent(ReleaseEvent),
  RepositoryEvent(RepositoryEvent),
  RepositoryDispatchEvent(RepositoryDispatchEvent),
  RepositoryImportEvent(RepositoryImportEvent),
  RepositoryVulnerabilityAlertEvent(RepositoryVulnerabilityAlertEvent),
  SecretScanningAlertEvent(SecretScanningAlertEvent),
  SecurityAdvisoryEvent(SecurityAdvisoryEvent),
  SponsorshipEvent(SponsorshipEvent),
  StarEvent(StarEvent),
  StatusEvent(StatusEvent),
  TeamEvent(TeamEvent),
  TeamAddEvent(TeamAddEvent),
  WatchEvent(WatchEvent),
  WorkflowDispatchEvent(WorkflowDispatchEvent),
  WorkflowJobEvent(WorkflowJobEvent),
  WorkflowRunEvent(WorkflowRunEvent),
}

// ts type alias
pub enum BranchProtectionRuleEvent {
  BranchProtectionRuleCreatedEvent(BranchProtectionRuleCreatedEvent),
  BranchProtectionRuleDeletedEvent(BranchProtectionRuleDeletedEvent),
  BranchProtectionRuleEditedEvent(BranchProtectionRuleEditedEvent),
}

// ts type alias
pub enum BranchProtectionRuleEnforcementLevel {
  off,
  non_admins,
  everyone,
}

// ts type alias
pub type BranchProtectionRuleNumber = usize;
// ts type alias
pub type BranchProtectionRuleBoolean = bool;
// ts type alias
pub type BranchProtectionRuleArray = Vec<String>;
// ts type alias
pub enum CheckRunEvent {
  CheckRunCompletedEvent(CheckRunCompletedEvent),
  CheckRunCreatedEvent(CheckRunCreatedEvent),
  CheckRunRequestedActionEvent(CheckRunRequestedActionEvent),
  CheckRunRerequestedEvent(CheckRunRerequestedEvent),
}

// ts type alias
pub enum CheckSuiteEvent {
  CheckSuiteCompletedEvent(CheckSuiteCompletedEvent),
  CheckSuiteRequestedEvent(CheckSuiteRequestedEvent),
  CheckSuiteRerequestedEvent(CheckSuiteRerequestedEvent),
}

// ts type alias
pub enum CodeScanningAlertEvent {
  CodeScanningAlertAppearedInBranchEvent(CodeScanningAlertAppearedInBranchEvent),
  CodeScanningAlertClosedByUserEvent(CodeScanningAlertClosedByUserEvent),
  CodeScanningAlertCreatedEvent(CodeScanningAlertCreatedEvent),
  CodeScanningAlertFixedEvent(CodeScanningAlertFixedEvent),
  CodeScanningAlertReopenedEvent(CodeScanningAlertReopenedEvent),
  CodeScanningAlertReopenedByUserEvent(CodeScanningAlertReopenedByUserEvent),
}

// ts type alias
pub type CommitCommentEvent = CommitCommentCreatedEvent;
// ts type alias
pub enum AuthorAssociation {
  COLLABORATOR,
  CONTRIBUTOR,
  FIRST_TIMER,
  FIRST_TIME_CONTRIBUTOR,
  MANNEQUIN,
  MEMBER,
  NONE,
  OWNER,
}

// ts type alias
pub enum DependabotAlertEvent {
  DependabotAlertCreatedEvent(DependabotAlertCreatedEvent),
  DependabotAlertDismissedEvent(DependabotAlertDismissedEvent),
  DependabotAlertFixedEvent(DependabotAlertFixedEvent),
  DependabotAlertReintroducedEvent(DependabotAlertReintroducedEvent),
  DependabotAlertReopenedEvent(DependabotAlertReopenedEvent),
}

// ts type alias
pub enum DeployKeyEvent {
  DeployKeyCreatedEvent(DeployKeyCreatedEvent),
  DeployKeyDeletedEvent(DeployKeyDeletedEvent),
}

// ts type alias
pub type DeploymentEvent = DeploymentCreatedEvent;
// ts type alias
pub type DeploymentStatusEvent = DeploymentStatusCreatedEvent;
// ts type alias
pub enum DiscussionEvent {
  DiscussionAnsweredEvent(DiscussionAnsweredEvent),
  DiscussionCategoryChangedEvent(DiscussionCategoryChangedEvent),
  DiscussionCreatedEvent(DiscussionCreatedEvent),
  DiscussionDeletedEvent(DiscussionDeletedEvent),
  DiscussionEditedEvent(DiscussionEditedEvent),
  DiscussionLabeledEvent(DiscussionLabeledEvent),
  DiscussionLockedEvent(DiscussionLockedEvent),
  DiscussionPinnedEvent(DiscussionPinnedEvent),
  DiscussionTransferredEvent(DiscussionTransferredEvent),
  DiscussionUnansweredEvent(DiscussionUnansweredEvent),
  DiscussionUnlabeledEvent(DiscussionUnlabeledEvent),
  DiscussionUnlockedEvent(DiscussionUnlockedEvent),
  DiscussionUnpinnedEvent(DiscussionUnpinnedEvent),
}

// ts type alias
pub enum DiscussionCommentEvent {
  DiscussionCommentCreatedEvent(DiscussionCommentCreatedEvent),
  DiscussionCommentDeletedEvent(DiscussionCommentDeletedEvent),
  DiscussionCommentEditedEvent(DiscussionCommentEditedEvent),
}

// ts type alias
pub type GithubAppAuthorizationEvent = GithubAppAuthorizationRevokedEvent;
// ts type alias
pub enum InstallationEvent {
  InstallationCreatedEvent(InstallationCreatedEvent),
  InstallationDeletedEvent(InstallationDeletedEvent),
  InstallationNewPermissionsAcceptedEvent(InstallationNewPermissionsAcceptedEvent),
  InstallationSuspendEvent(InstallationSuspendEvent),
  InstallationUnsuspendEvent(InstallationUnsuspendEvent),
}

// ts type alias
pub enum InstallationRepositoriesEvent {
  InstallationRepositoriesAddedEvent(InstallationRepositoriesAddedEvent),
  InstallationRepositoriesRemovedEvent(InstallationRepositoriesRemovedEvent),
}

// ts type alias
pub enum IssueCommentEvent {
  IssueCommentCreatedEvent(IssueCommentCreatedEvent),
  IssueCommentDeletedEvent(IssueCommentDeletedEvent),
  IssueCommentEditedEvent(IssueCommentEditedEvent),
}

// ts type alias
pub enum IssuesEvent {
  IssuesAssignedEvent(IssuesAssignedEvent),
  IssuesClosedEvent(IssuesClosedEvent),
  IssuesDeletedEvent(IssuesDeletedEvent),
  IssuesDemilestonedEvent(IssuesDemilestonedEvent),
  IssuesEditedEvent(IssuesEditedEvent),
  IssuesLabeledEvent(IssuesLabeledEvent),
  IssuesLockedEvent(IssuesLockedEvent),
  IssuesMilestonedEvent(IssuesMilestonedEvent),
  IssuesOpenedEvent(IssuesOpenedEvent),
  IssuesPinnedEvent(IssuesPinnedEvent),
  IssuesReopenedEvent(IssuesReopenedEvent),
  IssuesTransferredEvent(IssuesTransferredEvent),
  IssuesUnassignedEvent(IssuesUnassignedEvent),
  IssuesUnlabeledEvent(IssuesUnlabeledEvent),
  IssuesUnlockedEvent(IssuesUnlockedEvent),
  IssuesUnpinnedEvent(IssuesUnpinnedEvent),
}

// ts type alias
pub enum LabelEvent {
  LabelCreatedEvent(LabelCreatedEvent),
  LabelDeletedEvent(LabelDeletedEvent),
  LabelEditedEvent(LabelEditedEvent),
}

// ts type alias
pub enum MarketplacePurchaseEvent {
  MarketplacePurchaseCancelledEvent(MarketplacePurchaseCancelledEvent),
  MarketplacePurchaseChangedEvent(MarketplacePurchaseChangedEvent),
  MarketplacePurchasePendingChangeEvent(MarketplacePurchasePendingChangeEvent),
  MarketplacePurchasePendingChangeCancelledEvent(MarketplacePurchasePendingChangeCancelledEvent),
  MarketplacePurchasePurchasedEvent(MarketplacePurchasePurchasedEvent),
}

// ts type alias
pub enum MemberEvent {
  MemberAddedEvent(MemberAddedEvent),
  MemberEditedEvent(MemberEditedEvent),
  MemberRemovedEvent(MemberRemovedEvent),
}

// ts type alias
pub enum MembershipEvent {
  MembershipAddedEvent(MembershipAddedEvent),
  MembershipRemovedEvent(MembershipRemovedEvent),
}

// ts type alias
pub type MergeGroupEvent = MergeGroupChecksRequestedEvent;
// ts type alias
pub type MetaEvent = MetaDeletedEvent;
// ts type alias
// ts type alias
pub enum MilestoneEvent {
  MilestoneClosedEvent(MilestoneClosedEvent),
  MilestoneCreatedEvent(MilestoneCreatedEvent),
  MilestoneDeletedEvent(MilestoneDeletedEvent),
  MilestoneEditedEvent(MilestoneEditedEvent),
  MilestoneOpenedEvent(MilestoneOpenedEvent),
}

// ts type alias
pub enum OrgBlockEvent {
  OrgBlockBlockedEvent(OrgBlockBlockedEvent),
  OrgBlockUnblockedEvent(OrgBlockUnblockedEvent),
}

// ts type alias
pub enum OrganizationEvent {
  OrganizationDeletedEvent(OrganizationDeletedEvent),
  OrganizationMemberAddedEvent(OrganizationMemberAddedEvent),
  OrganizationMemberInvitedEvent(OrganizationMemberInvitedEvent),
  OrganizationMemberRemovedEvent(OrganizationMemberRemovedEvent),
  OrganizationRenamedEvent(OrganizationRenamedEvent),
}

// ts type alias
pub enum PackageEvent {
  PackagePublishedEvent(PackagePublishedEvent),
  PackageUpdatedEvent(PackageUpdatedEvent),
}

// ts type alias
pub enum ProjectEvent {
  ProjectClosedEvent(ProjectClosedEvent),
  ProjectCreatedEvent(ProjectCreatedEvent),
  ProjectDeletedEvent(ProjectDeletedEvent),
  ProjectEditedEvent(ProjectEditedEvent),
  ProjectReopenedEvent(ProjectReopenedEvent),
}

// ts type alias
pub enum ProjectCardEvent {
  ProjectCardConvertedEvent(ProjectCardConvertedEvent),
  ProjectCardCreatedEvent(ProjectCardCreatedEvent),
  ProjectCardDeletedEvent(ProjectCardDeletedEvent),
  ProjectCardEditedEvent(ProjectCardEditedEvent),
  ProjectCardMovedEvent(ProjectCardMovedEvent),
}

// ts type alias
pub enum ProjectColumnEvent {
  ProjectColumnCreatedEvent(ProjectColumnCreatedEvent),
  ProjectColumnDeletedEvent(ProjectColumnDeletedEvent),
  ProjectColumnEditedEvent(ProjectColumnEditedEvent),
  ProjectColumnMovedEvent(ProjectColumnMovedEvent),
}

// ts type alias
pub enum ProjectsV2ItemEvent {
  ProjectsV2ItemArchivedEvent(ProjectsV2ItemArchivedEvent),
  ProjectsV2ItemConvertedEvent(ProjectsV2ItemConvertedEvent),
  ProjectsV2ItemCreatedEvent(ProjectsV2ItemCreatedEvent),
  ProjectsV2ItemDeletedEvent(ProjectsV2ItemDeletedEvent),
  ProjectsV2ItemEditedEvent(ProjectsV2ItemEditedEvent),
  ProjectsV2ItemReorderedEvent(ProjectsV2ItemReorderedEvent),
  ProjectsV2ItemRestoredEvent(ProjectsV2ItemRestoredEvent),
}

// ts type alias
pub enum PullRequestEvent {
  PullRequestAssignedEvent(PullRequestAssignedEvent),
  PullRequestAutoMergeDisabledEvent(PullRequestAutoMergeDisabledEvent),
  PullRequestAutoMergeEnabledEvent(PullRequestAutoMergeEnabledEvent),
  PullRequestClosedEvent(PullRequestClosedEvent),
  PullRequestConvertedToDraftEvent(PullRequestConvertedToDraftEvent),
  PullRequestDequeuedEvent(PullRequestDequeuedEvent),
  PullRequestEditedEvent(PullRequestEditedEvent),
  PullRequestLabeledEvent(PullRequestLabeledEvent),
  PullRequestLockedEvent(PullRequestLockedEvent),
  PullRequestOpenedEvent(PullRequestOpenedEvent),
  PullRequestQueuedEvent(PullRequestQueuedEvent),
  PullRequestReadyForReviewEvent(PullRequestReadyForReviewEvent),
  PullRequestReopenedEvent(PullRequestReopenedEvent),
  PullRequestReviewRequestRemovedEvent(PullRequestReviewRequestRemovedEvent),
  PullRequestReviewRequestedEvent(PullRequestReviewRequestedEvent),
  PullRequestSynchronizeEvent(PullRequestSynchronizeEvent),
  PullRequestUnassignedEvent(PullRequestUnassignedEvent),
  PullRequestUnlabeledEvent(PullRequestUnlabeledEvent),
  PullRequestUnlockedEvent(PullRequestUnlockedEvent),
}

// ts type alias
// ts type alias
// ts type alias
pub enum PullRequestReviewEvent {
  PullRequestReviewDismissedEvent(PullRequestReviewDismissedEvent),
  PullRequestReviewEditedEvent(PullRequestReviewEditedEvent),
  PullRequestReviewSubmittedEvent(PullRequestReviewSubmittedEvent),
}

// ts type alias
pub enum PullRequestReviewCommentEvent {
  PullRequestReviewCommentCreatedEvent(PullRequestReviewCommentCreatedEvent),
  PullRequestReviewCommentDeletedEvent(PullRequestReviewCommentDeletedEvent),
  PullRequestReviewCommentEditedEvent(PullRequestReviewCommentEditedEvent),
}

// ts type alias
pub enum PullRequestReviewThreadEvent {
  PullRequestReviewThreadResolvedEvent(PullRequestReviewThreadResolvedEvent),
  PullRequestReviewThreadUnresolvedEvent(PullRequestReviewThreadUnresolvedEvent),
}

// ts type alias
pub enum RegistryPackageEvent {
  RegistryPackagePublishedEvent(RegistryPackagePublishedEvent),
  RegistryPackageUpdatedEvent(RegistryPackageUpdatedEvent),
}

// ts type alias
pub enum ReleaseEvent {
  ReleaseCreatedEvent(ReleaseCreatedEvent),
  ReleaseDeletedEvent(ReleaseDeletedEvent),
  ReleaseEditedEvent(ReleaseEditedEvent),
  ReleasePrereleasedEvent(ReleasePrereleasedEvent),
  ReleasePublishedEvent(ReleasePublishedEvent),
  ReleaseReleasedEvent(ReleaseReleasedEvent),
  ReleaseUnpublishedEvent(ReleaseUnpublishedEvent),
}

// ts type alias
pub enum RepositoryEvent {
  RepositoryArchivedEvent(RepositoryArchivedEvent),
  RepositoryCreatedEvent(RepositoryCreatedEvent),
  RepositoryDeletedEvent(RepositoryDeletedEvent),
  RepositoryEditedEvent(RepositoryEditedEvent),
  RepositoryPrivatizedEvent(RepositoryPrivatizedEvent),
  RepositoryPublicizedEvent(RepositoryPublicizedEvent),
  RepositoryRenamedEvent(RepositoryRenamedEvent),
  RepositoryTransferredEvent(RepositoryTransferredEvent),
  RepositoryUnarchivedEvent(RepositoryUnarchivedEvent),
}

// ts type alias
pub enum RepositoryVulnerabilityAlertEvent {
  RepositoryVulnerabilityAlertCreateEvent(RepositoryVulnerabilityAlertCreateEvent),
  RepositoryVulnerabilityAlertDismissEvent(RepositoryVulnerabilityAlertDismissEvent),
  RepositoryVulnerabilityAlertReopenEvent(RepositoryVulnerabilityAlertReopenEvent),
  RepositoryVulnerabilityAlertResolveEvent(RepositoryVulnerabilityAlertResolveEvent),
}

// ts type alias
pub enum SecretScanningAlertEvent {
  SecretScanningAlertCreatedEvent(SecretScanningAlertCreatedEvent),
  SecretScanningAlertReopenedEvent(SecretScanningAlertReopenedEvent),
  SecretScanningAlertResolvedEvent(SecretScanningAlertResolvedEvent),
}

// ts type alias
pub enum SecurityAdvisoryEvent {
  SecurityAdvisoryPerformedEvent(SecurityAdvisoryPerformedEvent),
  SecurityAdvisoryPublishedEvent(SecurityAdvisoryPublishedEvent),
  SecurityAdvisoryUpdatedEvent(SecurityAdvisoryUpdatedEvent),
  SecurityAdvisoryWithdrawnEvent(SecurityAdvisoryWithdrawnEvent),
}

// ts type alias
pub enum SponsorshipEvent {
  SponsorshipCancelledEvent(SponsorshipCancelledEvent),
  SponsorshipCreatedEvent(SponsorshipCreatedEvent),
  SponsorshipEditedEvent(SponsorshipEditedEvent),
  SponsorshipPendingCancellationEvent(SponsorshipPendingCancellationEvent),
  SponsorshipPendingTierChangeEvent(SponsorshipPendingTierChangeEvent),
  SponsorshipTierChangedEvent(SponsorshipTierChangedEvent),
}

// ts type alias
pub enum StarEvent {
  StarCreatedEvent(StarCreatedEvent),
  StarDeletedEvent(StarDeletedEvent),
}

// ts type alias
pub enum TeamEvent {
  TeamAddedToRepositoryEvent(TeamAddedToRepositoryEvent),
  TeamCreatedEvent(TeamCreatedEvent),
  TeamDeletedEvent(TeamDeletedEvent),
  TeamEditedEvent(TeamEditedEvent),
  TeamRemovedFromRepositoryEvent(TeamRemovedFromRepositoryEvent),
}

// ts type alias
pub type WatchEvent = WatchStartedEvent;
// ts type alias
pub enum WorkflowJobEvent {
  WorkflowJobCompletedEvent(WorkflowJobCompletedEvent),
  WorkflowJobInProgressEvent(WorkflowJobInProgressEvent),
  WorkflowJobQueuedEvent(WorkflowJobQueuedEvent),
}

// ts type alias
pub enum WorkflowStep {
  WorkflowStepInProgress(WorkflowStepInProgress),
  WorkflowStepCompleted(WorkflowStepCompleted),
}

// ts type alias
pub enum WorkflowRunEvent {
  WorkflowRunCompletedEvent(WorkflowRunCompletedEvent),
  WorkflowRunInProgressEvent(WorkflowRunInProgressEvent),
  WorkflowRunRequestedEvent(WorkflowRunRequestedEvent),
}

// ts interface
pub struct BranchProtectionRuleCreatedEvent {
  // pub action: UnknwonLiteral,
  pub rule: BranchProtectionRule,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct BranchProtectionRule {
  pub id: usize,
  pub repository_id: usize,
  pub name: String,
  pub created_at: String,
  pub updated_at: String,
  pub pull_request_reviews_enforcement_level: BranchProtectionRuleEnforcementLevel,
  pub required_approving_review_count: BranchProtectionRuleNumber,
  pub dismiss_stale_reviews_on_push: BranchProtectionRuleBoolean,
  pub require_code_owner_review: BranchProtectionRuleBoolean,
  pub authorized_dismissal_actors_only: BranchProtectionRuleBoolean,
  pub ignore_approvals_from_contributors: BranchProtectionRuleBoolean,
  pub required_status_checks: BranchProtectionRuleArray,
  pub required_status_checks_enforcement_level: BranchProtectionRuleEnforcementLevel,
  pub strict_required_status_checks_policy: BranchProtectionRuleBoolean,
  pub signature_requirement_enforcement_level: BranchProtectionRuleEnforcementLevel,
  pub linear_history_requirement_enforcement_level: BranchProtectionRuleEnforcementLevel,
  pub admin_enforced: BranchProtectionRuleBoolean,
  pub allow_force_pushes_enforcement_level: BranchProtectionRuleEnforcementLevel,
  pub allow_deletions_enforcement_level: BranchProtectionRuleEnforcementLevel,
  pub merge_queue_enforcement_level: BranchProtectionRuleEnforcementLevel,
  pub required_deployments_enforcement_level: BranchProtectionRuleEnforcementLevel,
  pub required_conversation_resolution_level: BranchProtectionRuleEnforcementLevel,
  pub authorized_actors_only: BranchProtectionRuleBoolean,
  pub authorized_actor_names: BranchProtectionRuleArray,
}
// ts interface
pub struct Repository {
  pub id: usize,
  pub node_id: String,
  pub name: String,
  pub full_name: String,
  pub private: bool,
  pub owner: User,
  pub html_url: String,
  pub description: Option<String>,
  pub fork: bool,
  pub url: String,
  pub forks_url: String,
  pub keys_url: String,
  pub collaborators_url: String,
  pub teams_url: String,
  pub hooks_url: String,
  pub issue_events_url: String,
  pub events_url: String,
  pub assignees_url: String,
  pub branches_url: String,
  pub tags_url: String,
  pub blobs_url: String,
  pub git_tags_url: String,
  pub git_refs_url: String,
  pub trees_url: String,
  pub statuses_url: String,
  pub languages_url: String,
  pub stargazers_url: String,
  pub contributors_url: String,
  pub subscribers_url: String,
  pub subscription_url: String,
  pub commits_url: String,
  pub git_commits_url: String,
  pub comments_url: String,
  pub issue_comment_url: String,
  pub contents_url: String,
  pub compare_url: String,
  pub merges_url: String,
  pub archive_url: String,
  pub downloads_url: String,
  pub issues_url: String,
  pub pulls_url: String,
  pub milestones_url: String,
  pub notifications_url: String,
  pub labels_url: String,
  pub releases_url: String,
  pub deployments_url: String,
  // pub created_at: UnknwonUnion,
  pub updated_at: String,
  // pub pushed_at: Option<UnknwonUnion>,
  pub git_url: String,
  pub ssh_url: String,
  pub clone_url: String,
  pub svn_url: String,
  pub homepage: Option<String>,
  pub size: usize,
  pub stargazers_count: usize,
  pub watchers_count: usize,
  pub language: Option<String>,
  pub has_issues: bool,
  pub has_projects: bool,
  pub has_downloads: bool,
  pub has_wiki: bool,
  pub has_pages: bool,
  pub forks_count: usize,
  pub mirror_url: Option<String>,
  pub archived: bool,
  pub open_issues_count: usize,
  pub license: Option<License>,
  pub forks: usize,
  pub open_issues: usize,
  pub watchers: usize,
  pub default_branch: String,
  pub is_template: bool,
  pub web_commit_signoff_required: bool,
  pub topics: Vec<String>,
  pub visibility: String,
}
// ts interface
pub struct User {
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
  pub type_: String,
  pub site_admin: bool,
}
// ts interface
pub struct License {
  pub key: String,
  pub name: String,
  pub spdx_id: String,
  pub url: Option<String>,
  pub node_id: String,
}
// ts interface
pub struct InstallationLite {
  pub id: usize,
  pub node_id: String,
}
// ts interface
pub struct Organization {
  pub login: String,
  pub id: usize,
  pub node_id: String,
  pub url: String,
  pub repos_url: String,
  pub events_url: String,
  pub hooks_url: String,
  pub issues_url: String,
  pub members_url: String,
  pub public_members_url: String,
  pub avatar_url: String,
  pub description: Option<String>,
}
// ts interface
pub struct BranchProtectionRuleDeletedEvent {
  // pub action: UnknwonLiteral,
  pub rule: BranchProtectionRule,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct BranchProtectionRuleEditedEvent {
  // pub action: UnknwonLiteral,
  pub rule: BranchProtectionRule,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct CheckRunCompletedEvent {
  // pub action: UnknwonLiteral,
  // pub check_run: Unknwon,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct CheckRunPullRequest {
  pub url: String,
  pub id: usize,
  pub number: usize,
  // pub head: Unknwon,
  // pub base: Unknwon,
}
// ts interface
pub struct RepoRef {
  pub id: usize,
  pub url: String,
  pub name: String,
}
// ts interface
pub struct CheckRunDeployment {
  pub url: String,
  pub id: usize,
  pub node_id: String,
  pub task: String,
  pub original_environment: String,
  pub environment: String,
  pub description: Option<String>,
  pub created_at: String,
  pub updated_at: String,
  pub statuses_url: String,
  pub repository_url: String,
}
// ts interface
pub struct App {
  pub id: usize,
  pub node_id: String,
  pub owner: User,
  pub name: String,
  pub description: Option<String>,
  pub external_url: String,
  pub html_url: String,
  pub created_at: String,
  pub updated_at: String,
}
// ts interface
pub struct CheckRunCreatedEvent {
  // pub action: UnknwonLiteral,
  // pub check_run: Unknwon,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct CheckRunRequestedActionEvent {
  // pub action: UnknwonLiteral,
  // pub check_run: Unknwon,
  // pub requested_action: Unknwon,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct CheckRunRerequestedEvent {
  // pub action: UnknwonLiteral,
  // pub check_run: Unknwon,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct CheckSuiteCompletedEvent {
  // pub action: UnknwonLiteral,
  // pub check_suite: Unknwon,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct SimpleCommit {
  pub id: String,
  pub tree_id: String,
  pub message: String,
  pub timestamp: String,
  pub author: Committer,
  pub committer: Committer,
}
// ts interface
pub struct Committer {
  pub name: String,
  pub email: Option<String>,
}
// ts interface
pub struct CheckSuiteRequestedEvent {
  // pub action: UnknwonLiteral,
  // pub check_suite: Unknwon,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct CheckSuiteRerequestedEvent {
  // pub action: UnknwonLiteral,
  // pub check_suite: Unknwon,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct CodeScanningAlertAppearedInBranchEvent {
  // pub action: UnknwonLiteral,
  // pub alert: Unknwon,
  pub ref_: String,
  pub commit_oid: String,
  pub repository: Repository,
  pub sender: GitHubOrg,
}
// ts interface
pub struct AlertInstance {
  pub ref_: String,
  pub analysis_key: String,
  pub environment: String,
  pub state: String,
}
// ts interface
pub struct GitHubOrg {
  // pub login: UnknwonLiteral,
  // pub id: UnknwonLiteral,
  // pub node_id: UnknwonLiteral,
  // pub avatar_url: UnknwonLiteral,
  // pub gravatar_id: UnknwonLiteral,
  // pub url: UnknwonLiteral,
  // pub html_url: UnknwonLiteral,
  // pub followers_url: UnknwonLiteral,
  // pub following_url: UnknwonLiteral,
  // pub gists_url: UnknwonLiteral,
  // pub starred_url: UnknwonLiteral,
  // pub subscriptions_url: UnknwonLiteral,
  // pub organizations_url: UnknwonLiteral,
  // pub repos_url: UnknwonLiteral,
  // pub events_url: UnknwonLiteral,
  // pub received_events_url: UnknwonLiteral,
  // pub type_: UnknwonLiteral,
  // pub site_admin: UnknwonLiteral,
}
// ts interface
pub struct CodeScanningAlertClosedByUserEvent {
  // pub action: UnknwonLiteral,
  // pub alert: Unknwon,
  pub ref_: String,
  pub commit_oid: String,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct CodeScanningAlertCreatedEvent {
  // pub action: UnknwonLiteral,
  // pub alert: Unknwon,
  pub ref_: String,
  pub commit_oid: String,
  pub repository: Repository,
  pub sender: GitHubOrg,
}
// ts interface
pub struct CodeScanningAlertFixedEvent {
  // pub action: UnknwonLiteral,
  // pub alert: Unknwon,
  pub ref_: String,
  pub commit_oid: String,
  pub repository: Repository,
  pub sender: GitHubOrg,
}
// ts interface
pub struct CodeScanningAlertReopenedEvent {
  // pub action: UnknwonLiteral,
  // pub alert: Unknwon,
  pub ref_: String,
  pub commit_oid: String,
  pub repository: Repository,
  pub sender: GitHubOrg,
}
// ts interface
pub struct CodeScanningAlertReopenedByUserEvent {
  // pub action: UnknwonLiteral,
  // pub alert: Unknwon,
  pub ref_: String,
  pub commit_oid: String,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct CommitCommentCreatedEvent {
  // pub action: UnknwonLiteral,
  // pub comment: Unknwon,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct CreateEvent {
  pub ref_: String,
  pub ref_type: String,
  pub master_branch: String,
  pub description: Option<String>,
  pub pusher_type: String,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct DeleteEvent {
  pub ref_: String,
  pub ref_type: String,
  pub pusher_type: String,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct DependabotAlertCreatedEvent {
  // pub action: UnknwonLiteral,
  // pub alert: UnknwonIntersection,
  pub repository: Repository,
  pub sender: GitHubOrg,
}
// ts interface
pub struct DependabotAlert {
  pub number: usize,
  pub state: String,
  // pub dependency: Unknwon,
  // pub security_advisory: Unknwon,
  // pub security_vulnerability: Unknwon,
  pub url: String,
  pub html_url: String,
  pub created_at: String,
  pub updated_at: String,
  pub dismissed_at: Option<String>,
  pub dismissed_by: Option<User>,
  pub dismissed_reason: Option<String>,
  pub dismissed_comment: Option<String>,
  pub fixed_at: Option<String>,
}
// ts interface
pub struct DependabotAlertPackage {
  pub name: String,
  pub ecosystem: String,
}
// ts interface
pub struct SecurityAdvisoryCvss {
  pub score: usize,
  pub vector_string: Option<String>,
}
// ts interface
pub struct SecurityAdvisoryCwes {
  pub cwe_id: String,
  pub name: String,
}
// ts interface
pub struct DependabotAlertDismissedEvent {
  // pub action: UnknwonLiteral,
  // pub alert: UnknwonIntersection,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct DependabotAlertFixedEvent {
  // pub action: UnknwonLiteral,
  // pub alert: UnknwonIntersection,
  pub repository: Repository,
  pub sender: GitHubOrg,
}
// ts interface
pub struct DependabotAlertReintroducedEvent {
  // pub action: UnknwonLiteral,
  pub alert: DependabotAlert,
  pub repository: Repository,
  pub sender: GitHubOrg,
}
// ts interface
pub struct DependabotAlertReopenedEvent {
  // pub action: UnknwonLiteral,
  pub alert: DependabotAlert,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct DeployKeyCreatedEvent {
  // pub action: UnknwonLiteral,
  // pub key: Unknwon,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct DeployKeyDeletedEvent {
  // pub action: UnknwonLiteral,
  // pub key: Unknwon,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct DeploymentCreatedEvent {
  // pub action: UnknwonLiteral,
  pub deployment: Deployment,
  pub workflow: Option<Workflow>,
  pub workflow_run: Option<DeploymentWorkflowRun>,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct Deployment {
  pub url: String,
  pub id: usize,
  pub node_id: String,
  pub sha: String,
  pub ref_: String,
  pub task: String,
  // pub payload: Unknwon,
  pub original_environment: String,
  pub environment: String,
  pub description: Option<String>,
  pub creator: User,
  pub created_at: String,
  pub updated_at: String,
  pub statuses_url: String,
  pub repository_url: String,
}
// ts interface
pub struct Workflow {
  pub badge_url: String,
  pub created_at: String,
  pub html_url: String,
  pub id: usize,
  pub name: String,
  pub node_id: String,
  pub path: String,
  pub state: String,
  pub updated_at: String,
  pub url: String,
}
// ts interface
pub struct DeploymentWorkflowRun {
  pub id: usize,
  pub name: String,
  pub node_id: String,
  pub head_branch: String,
  pub head_sha: String,
  pub run_number: usize,
  pub event: String,
  pub status: String,
  pub conclusion: Option<String>,
  pub workflow_id: usize,
  pub check_suite_id: usize,
  pub check_suite_node_id: String,
  pub url: String,
  pub html_url: String,
  pub pull_requests: Vec<CheckRunPullRequest>,
  pub created_at: String,
  pub updated_at: String,
  pub actor: User,
  pub triggering_actor: User,
  pub run_attempt: usize,
  pub run_started_at: String,
}
// ts interface
pub struct ReferencedWorkflow {
  pub path: String,
  pub sha: String,
}
// ts interface
pub struct DeploymentStatusCreatedEvent {
  // pub action: UnknwonLiteral,
  // pub deployment_status: Unknwon,
  pub deployment: Deployment,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct DiscussionAnsweredEvent {
  // pub action: UnknwonLiteral,
  // pub discussion: UnknwonIntersection,
  // pub answer: Unknwon,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct Discussion {
  pub repository_url: String,
  // pub category: Unknwon,
  pub answer_html_url: Option<String>,
  pub answer_chosen_at: Option<String>,
  pub answer_chosen_by: Option<User>,
  pub html_url: String,
  pub id: usize,
  pub node_id: String,
  pub number: usize,
  pub title: String,
  pub user: User,
  pub state: String,
  pub locked: bool,
  pub comments: usize,
  pub created_at: String,
  pub updated_at: String,
  pub author_association: AuthorAssociation,
  pub active_lock_reason: Option<String>,
  pub body: String,
}
// ts interface
pub struct Reactions {
  pub url: String,
  pub total_count: usize,
  pub laugh: usize,
  pub hooray: usize,
  pub confused: usize,
  pub heart: usize,
  pub rocket: usize,
  pub eyes: usize,
}
// ts interface
pub struct DiscussionCategoryChangedEvent {
  // pub changes: Unknwon,
  // pub action: UnknwonLiteral,
  pub discussion: Discussion,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct DiscussionCreatedEvent {
  // pub action: UnknwonLiteral,
  // pub discussion: UnknwonIntersection,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct DiscussionDeletedEvent {
  // pub action: UnknwonLiteral,
  pub discussion: Discussion,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct DiscussionEditedEvent {
  // pub action: UnknwonLiteral,
  pub discussion: Discussion,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct DiscussionLabeledEvent {
  // pub action: UnknwonLiteral,
  pub discussion: Discussion,
  pub label: Label,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct Label {
  pub id: usize,
  pub node_id: String,
  pub url: String,
  pub name: String,
  pub description: Option<String>,
  pub color: String,
  pub default: bool,
}
// ts interface
pub struct DiscussionLockedEvent {
  // pub action: UnknwonLiteral,
  // pub discussion: UnknwonIntersection,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct DiscussionPinnedEvent {
  // pub action: UnknwonLiteral,
  pub discussion: Discussion,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct DiscussionTransferredEvent {
  // pub changes: Unknwon,
  // pub action: UnknwonLiteral,
  pub discussion: Discussion,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct DiscussionUnansweredEvent {
  // pub action: UnknwonLiteral,
  // pub discussion: UnknwonIntersection,
  // pub old_answer: Unknwon,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct DiscussionUnlabeledEvent {
  // pub action: UnknwonLiteral,
  pub discussion: Discussion,
  pub label: Label,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct DiscussionUnlockedEvent {
  // pub action: UnknwonLiteral,
  // pub discussion: UnknwonIntersection,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct DiscussionUnpinnedEvent {
  // pub action: UnknwonLiteral,
  pub discussion: Discussion,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct DiscussionCommentCreatedEvent {
  // pub action: UnknwonLiteral,
  // pub comment: Unknwon,
  pub discussion: Discussion,
  pub repository: Repository,
  pub sender: User,
  pub installation: InstallationLite,
}
// ts interface
pub struct DiscussionCommentDeletedEvent {
  // pub action: UnknwonLiteral,
  // pub comment: Unknwon,
  pub discussion: Discussion,
  pub repository: Repository,
  pub sender: User,
  pub installation: InstallationLite,
}
// ts interface
pub struct DiscussionCommentEditedEvent {
  // pub changes: Unknwon,
  // pub action: UnknwonLiteral,
  // pub comment: Unknwon,
  pub discussion: Discussion,
  pub repository: Repository,
  pub sender: User,
  pub installation: InstallationLite,
}
// ts interface
pub struct ForkEvent {
  // pub forkee: UnknwonIntersection,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct GithubAppAuthorizationRevokedEvent {
  // pub action: UnknwonLiteral,
  pub sender: User,
}
// ts interface
pub struct GollumEvent {
  // pub pages: Vec<Unknwon>,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct InstallationCreatedEvent {
  // pub action: UnknwonLiteral,
  pub installation: Installation,
  pub sender: User,
}
// ts interface
pub struct Installation {
  pub id: usize,
  pub account: User,
  pub repository_selection: String,
  pub access_tokens_url: String,
  pub repositories_url: String,
  pub html_url: String,
  pub app_id: usize,
  pub target_id: usize,
  pub target_type: String,
  // pub permissions: Unknwon,
  // pub events: Vec<Unknwon>,
  // pub created_at: UnknwonUnion,
  // pub updated_at: UnknwonUnion,
  pub single_file_name: Option<String>,
  pub suspended_by: Option<User>,
  pub suspended_at: Option<String>,
}
// ts interface
pub struct InstallationDeletedEvent {
  // pub action: UnknwonLiteral,
  pub installation: Installation,
  pub sender: User,
}
// ts interface
pub struct InstallationNewPermissionsAcceptedEvent {
  // pub action: UnknwonLiteral,
  pub installation: Installation,
  pub sender: User,
}
// ts interface
pub struct InstallationSuspendEvent {
  // pub action: UnknwonLiteral,
  // pub installation: UnknwonIntersection,
  pub sender: User,
}
// ts interface
pub struct InstallationUnsuspendEvent {
  // pub action: UnknwonLiteral,
  // pub installation: UnknwonIntersection,
  pub sender: User,
}
// ts interface
pub struct InstallationRepositoriesAddedEvent {
  // pub action: UnknwonLiteral,
  pub installation: Installation,
  pub repository_selection: String,
  // pub repositories_added: Vec<Unknwon>,
  // pub repositories_removed: Unknwon,
  pub requester: Option<User>,
  pub sender: User,
}
// ts interface
pub struct InstallationRepositoriesRemovedEvent {
  // pub action: UnknwonLiteral,
  pub installation: Installation,
  pub repository_selection: String,
  // pub repositories_added: Unknwon,
  // pub repositories_removed: Vec<Unknwon>,
  pub requester: Option<User>,
  pub sender: User,
}
// ts interface
pub struct IssueCommentCreatedEvent {
  // pub action: UnknwonLiteral,
  // pub issue: UnknwonIntersection,
  pub comment: IssueComment,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct Issue {
  pub url: String,
  pub repository_url: String,
  pub labels_url: String,
  pub comments_url: String,
  pub events_url: String,
  pub html_url: String,
  pub id: usize,
  pub node_id: String,
  pub number: usize,
  pub title: String,
  pub user: User,
  pub assignees: Vec<User>,
  pub milestone: Option<Milestone>,
  pub comments: usize,
  pub created_at: String,
  pub updated_at: String,
  pub closed_at: Option<String>,
  pub author_association: AuthorAssociation,
  pub active_lock_reason: Option<String>,
  pub body: Option<String>,
  pub reactions: Reactions,
}
// ts interface
pub struct Milestone {
  pub url: String,
  pub html_url: String,
  pub labels_url: String,
  pub id: usize,
  pub node_id: String,
  pub number: usize,
  pub title: String,
  pub description: Option<String>,
  pub creator: User,
  pub open_issues: usize,
  pub closed_issues: usize,
  pub state: String,
  pub created_at: String,
  pub updated_at: String,
  pub due_on: Option<String>,
  pub closed_at: Option<String>,
}
// ts interface
pub struct IssueComment {
  pub url: String,
  pub html_url: String,
  pub issue_url: String,
  pub id: usize,
  pub node_id: String,
  pub user: User,
  pub created_at: String,
  pub updated_at: String,
  pub author_association: AuthorAssociation,
  pub body: String,
  pub reactions: Reactions,
  pub performed_via_github_app: Option<App>,
}
// ts interface
pub struct IssueCommentDeletedEvent {
  // pub action: UnknwonLiteral,
  // pub issue: UnknwonIntersection,
  pub comment: IssueComment,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct IssueCommentEditedEvent {
  // pub action: UnknwonLiteral,
  // pub changes: Unknwon,
  // pub issue: UnknwonIntersection,
  pub comment: IssueComment,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct IssuesAssignedEvent {
  // pub action: UnknwonLiteral,
  pub issue: Issue,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct IssuesClosedEvent {
  // pub action: UnknwonLiteral,
  // pub issue: UnknwonIntersection,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct IssuesDeletedEvent {
  // pub action: UnknwonLiteral,
  pub issue: Issue,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct IssuesDemilestonedEvent {
  // pub action: UnknwonLiteral,
  // pub issue: UnknwonIntersection,
  pub milestone: Milestone,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct IssuesEditedEvent {
  // pub action: UnknwonLiteral,
  pub issue: Issue,
  // pub changes: Unknwon,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct IssuesLabeledEvent {
  // pub action: UnknwonLiteral,
  pub issue: Issue,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct IssuesLockedEvent {
  // pub action: UnknwonLiteral,
  // pub issue: UnknwonIntersection,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct IssuesMilestonedEvent {
  // pub action: UnknwonLiteral,
  // pub issue: UnknwonIntersection,
  pub milestone: Milestone,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct IssuesOpenedEvent {
  // pub action: UnknwonLiteral,
  // pub issue: UnknwonIntersection,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct IssuesPinnedEvent {
  // pub action: UnknwonLiteral,
  pub issue: Issue,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct IssuesReopenedEvent {
  // pub action: UnknwonLiteral,
  // pub issue: UnknwonIntersection,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct IssuesTransferredEvent {
  // pub action: UnknwonLiteral,
  // pub changes: Unknwon,
  pub issue: Issue,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct IssuesUnassignedEvent {
  // pub action: UnknwonLiteral,
  pub issue: Issue,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct IssuesUnlabeledEvent {
  // pub action: UnknwonLiteral,
  pub issue: Issue,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct IssuesUnlockedEvent {
  // pub action: UnknwonLiteral,
  // pub issue: UnknwonIntersection,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct IssuesUnpinnedEvent {
  // pub action: UnknwonLiteral,
  pub issue: Issue,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct LabelCreatedEvent {
  // pub action: UnknwonLiteral,
  pub label: Label,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct LabelDeletedEvent {
  // pub action: UnknwonLiteral,
  pub label: Label,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct LabelEditedEvent {
  // pub action: UnknwonLiteral,
  pub label: Label,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct MarketplacePurchaseCancelledEvent {
  // pub action: UnknwonLiteral,
  pub effective_date: String,
  // pub sender: Unknwon,
  // pub marketplace_purchase: UnknwonIntersection,
}
// ts interface
pub struct MarketplacePurchase {
  // pub account: Unknwon,
  pub billing_cycle: String,
  pub unit_count: usize,
  pub on_free_trial: bool,
  pub free_trial_ends_on: Option<String>,
  // pub plan: Unknwon,
}
// ts interface
pub struct MarketplacePurchaseChangedEvent {
  // pub action: UnknwonLiteral,
  pub effective_date: String,
  // pub sender: Unknwon,
  // pub marketplace_purchase: UnknwonIntersection,
}
// ts interface
pub struct MarketplacePurchasePendingChangeEvent {
  // pub action: UnknwonLiteral,
  pub effective_date: String,
  // pub sender: Unknwon,
  // pub marketplace_purchase: UnknwonIntersection,
}
// ts interface
pub struct MarketplacePurchasePendingChangeCancelledEvent {
  // pub action: UnknwonLiteral,
  pub effective_date: String,
  // pub sender: Unknwon,
  // pub marketplace_purchase: UnknwonIntersection,
}
// ts interface
pub struct MarketplacePurchasePurchasedEvent {
  // pub action: UnknwonLiteral,
  pub effective_date: String,
  // pub sender: Unknwon,
  // pub marketplace_purchase: UnknwonIntersection,
}
// ts interface
pub struct MemberAddedEvent {
  // pub action: UnknwonLiteral,
  pub member: User,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct MemberEditedEvent {
  // pub action: UnknwonLiteral,
  pub member: User,
  // pub changes: Unknwon,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct MemberRemovedEvent {
  // pub action: UnknwonLiteral,
  pub member: User,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct MembershipAddedEvent {
  // pub action: UnknwonLiteral,
  // pub scope: UnknwonLiteral,
  pub member: User,
  pub sender: User,
  pub team: Team,
  pub organization: Organization,
}
// ts interface
pub struct Team {
  pub name: String,
  pub id: usize,
  pub node_id: String,
  pub slug: String,
  pub description: Option<String>,
  pub privacy: String,
  pub url: String,
  pub html_url: String,
  pub members_url: String,
  pub repositories_url: String,
  pub permission: String,
}
// ts interface
pub struct MembershipRemovedEvent {
  // pub action: UnknwonLiteral,
  pub scope: String,
  pub member: User,
  pub sender: User,
  // pub team: UnknwonUnion,
  pub organization: Organization,
}
// ts interface
pub struct MergeGroupChecksRequestedEvent {
  // pub action: UnknwonLiteral,
  // pub merge_group: Unknwon,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct MetaDeletedEvent {
  // pub action: UnknwonLiteral,
  pub hook_id: usize,
  // pub hook: Unknwon,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct MilestoneClosedEvent {
  // pub action: UnknwonLiteral,
  // pub milestone: UnknwonIntersection,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct MilestoneCreatedEvent {
  // pub action: UnknwonLiteral,
  // pub milestone: UnknwonIntersection,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct MilestoneDeletedEvent {
  // pub action: UnknwonLiteral,
  pub milestone: Milestone,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct MilestoneEditedEvent {
  // pub action: UnknwonLiteral,
  // pub changes: Unknwon,
  pub milestone: Milestone,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct MilestoneOpenedEvent {
  // pub action: UnknwonLiteral,
  // pub milestone: UnknwonIntersection,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct OrgBlockBlockedEvent {
  // pub action: UnknwonLiteral,
  pub blocked_user: User,
  pub sender: User,
  pub organization: Organization,
}
// ts interface
pub struct OrgBlockUnblockedEvent {
  // pub action: UnknwonLiteral,
  pub blocked_user: User,
  pub sender: User,
  pub organization: Organization,
}
// ts interface
pub struct OrganizationDeletedEvent {
  // pub action: UnknwonLiteral,
  pub sender: User,
  pub organization: Organization,
}
// ts interface
pub struct Membership {
  pub url: String,
  pub state: String,
  pub role: String,
  pub organization_url: String,
  pub user: User,
}
// ts interface
pub struct OrganizationMemberAddedEvent {
  // pub action: UnknwonLiteral,
  pub membership: Membership,
  pub sender: User,
  pub organization: Organization,
}
// ts interface
pub struct OrganizationMemberInvitedEvent {
  // pub action: UnknwonLiteral,
  // pub invitation: Unknwon,
  pub user: User,
  pub sender: User,
  pub organization: Organization,
}
// ts interface
pub struct OrganizationMemberRemovedEvent {
  // pub action: UnknwonLiteral,
  pub membership: Membership,
  pub sender: User,
  pub organization: Organization,
}
// ts interface
pub struct OrganizationRenamedEvent {
  // pub action: UnknwonLiteral,
  pub membership: Membership,
  pub sender: User,
  pub organization: Organization,
}
// ts interface
pub struct PackagePublishedEvent {
  // pub action: UnknwonLiteral,
  // pub package: Unknwon,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct PackageNPMMetadata {
}
// ts interface
pub struct PackageNugetMetadata {
}
// ts interface
pub struct PackageUpdatedEvent {
  // pub action: UnknwonLiteral,
  // pub package: Unknwon,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct PageBuildEvent {
  pub id: usize,
  // pub build: Unknwon,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct PingEvent {
  pub zen: String,
  pub hook_id: usize,
  // pub hook: Unknwon,
}
// ts interface
pub struct ProjectClosedEvent {
  // pub action: UnknwonLiteral,
  pub project: Project,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct Project {
  pub owner_url: String,
  pub url: String,
  pub html_url: String,
  pub columns_url: String,
  pub id: usize,
  pub node_id: String,
  pub name: String,
  pub body: Option<String>,
  pub number: usize,
  pub state: String,
  pub creator: User,
  pub created_at: String,
  pub updated_at: String,
}
// ts interface
pub struct ProjectCreatedEvent {
  // pub action: UnknwonLiteral,
  pub project: Project,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct ProjectDeletedEvent {
  // pub action: UnknwonLiteral,
  pub project: Project,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct ProjectEditedEvent {
  // pub action: UnknwonLiteral,
  pub project: Project,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct ProjectReopenedEvent {
  // pub action: UnknwonLiteral,
  pub project: Project,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct ProjectCardConvertedEvent {
  // pub action: UnknwonLiteral,
  // pub changes: Unknwon,
  pub project_card: ProjectCard,
  pub sender: User,
}
// ts interface
pub struct ProjectCard {
  pub url: String,
  pub project_url: String,
  pub column_url: String,
  pub column_id: usize,
  pub id: usize,
  pub node_id: String,
  pub note: Option<String>,
  pub archived: bool,
  pub creator: User,
  pub created_at: String,
  pub updated_at: String,
}
// ts interface
pub struct ProjectCardCreatedEvent {
  // pub action: UnknwonLiteral,
  pub project_card: ProjectCard,
  pub sender: User,
}
// ts interface
pub struct ProjectCardDeletedEvent {
  // pub action: UnknwonLiteral,
  pub project_card: ProjectCard,
  pub sender: User,
}
// ts interface
pub struct ProjectCardEditedEvent {
  // pub action: UnknwonLiteral,
  // pub changes: Unknwon,
  pub project_card: ProjectCard,
  pub sender: User,
}
// ts interface
pub struct ProjectCardMovedEvent {
  // pub action: UnknwonLiteral,
  // pub project_card: UnknwonIntersection,
  pub sender: User,
}
// ts interface
pub struct ProjectColumnCreatedEvent {
  // pub action: UnknwonLiteral,
  pub project_column: ProjectColumn,
  pub sender: User,
}
// ts interface
pub struct ProjectColumn {
  pub url: String,
  pub project_url: String,
  pub cards_url: String,
  pub id: usize,
  pub node_id: String,
  pub name: String,
  pub created_at: String,
  pub updated_at: String,
}
// ts interface
pub struct ProjectColumnDeletedEvent {
  // pub action: UnknwonLiteral,
  pub project_column: ProjectColumn,
  pub sender: User,
}
// ts interface
pub struct ProjectColumnEditedEvent {
  // pub action: UnknwonLiteral,
  // pub changes: Unknwon,
  pub project_column: ProjectColumn,
  pub sender: User,
}
// ts interface
pub struct ProjectColumnMovedEvent {
  // pub action: UnknwonLiteral,
  pub project_column: ProjectColumn,
  pub sender: User,
}
// ts interface
pub struct ProjectsV2ItemArchivedEvent {
  // pub changes: Unknwon,
  // pub action: UnknwonLiteral,
  // pub projects_v2_item: UnknwonIntersection,
  pub sender: User,
}
// ts interface
pub struct ProjectsV2Item {
  pub id: usize,
  pub node_id: String,
  pub project_node_id: String,
  pub content_node_id: String,
  pub content_type: String,
  pub creator: User,
  pub created_at: String,
  pub updated_at: String,
  pub archived_at: Option<String>,
}
// ts interface
pub struct ProjectsV2ItemConvertedEvent {
  // pub changes: Unknwon,
  // pub action: UnknwonLiteral,
  // pub projects_v2_item: UnknwonIntersection,
  pub sender: User,
}
// ts interface
pub struct ProjectsV2ItemCreatedEvent {
  // pub action: UnknwonLiteral,
  // pub projects_v2_item: UnknwonIntersection,
  pub sender: User,
}
// ts interface
pub struct ProjectsV2ItemDeletedEvent {
  // pub action: UnknwonLiteral,
  pub projects_v2_item: ProjectsV2Item,
  pub sender: User,
}
// ts interface
pub struct ProjectsV2ItemEditedEvent {
  // pub changes: Unknwon,
  // pub action: UnknwonLiteral,
  pub projects_v2_item: ProjectsV2Item,
  pub sender: User,
}
// ts interface
pub struct ProjectsV2ItemReorderedEvent {
  // pub changes: Unknwon,
  // pub action: UnknwonLiteral,
  pub projects_v2_item: ProjectsV2Item,
  pub sender: User,
}
// ts interface
pub struct ProjectsV2ItemRestoredEvent {
  // pub changes: Unknwon,
  // pub action: UnknwonLiteral,
  // pub projects_v2_item: UnknwonIntersection,
  pub sender: User,
}
// ts interface
pub struct PublicEvent {
  // pub repository: UnknwonIntersection,
  pub sender: User,
}
// ts interface
pub struct PullRequestAssignedEvent {
  // pub action: UnknwonLiteral,
  pub number: usize,
  pub pull_request: PullRequest,
  pub assignee: User,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct PullRequest {
  pub url: String,
  pub id: usize,
  pub node_id: String,
  pub html_url: String,
  pub diff_url: String,
  pub patch_url: String,
  pub issue_url: String,
  pub number: usize,
  pub state: String,
  pub locked: bool,
  pub title: String,
  pub user: User,
  pub body: Option<String>,
  pub created_at: String,
  pub updated_at: String,
  pub closed_at: Option<String>,
  pub merged_at: Option<String>,
  pub merge_commit_sha: Option<String>,
  pub assignee: Option<User>,
  pub assignees: Vec<User>,
  // pub requested_reviewers: Vec<Unknwon>,
  pub requested_teams: Vec<Team>,
  pub labels: Vec<Label>,
  pub milestone: Option<Milestone>,
  pub commits_url: String,
  pub review_comments_url: String,
  pub review_comment_url: String,
  pub comments_url: String,
  pub statuses_url: String,
  // pub head: Unknwon,
  // pub base: Unknwon,
  // pub _links: Unknwon,
  pub author_association: AuthorAssociation,
  pub auto_merge: Option<PullRequestAutoMerge>,
  pub active_lock_reason: Option<String>,
  pub draft: bool,
  pub merged: Option<bool>,
  pub mergeable: Option<bool>,
  pub rebaseable: Option<bool>,
  pub mergeable_state: String,
  pub merged_by: Option<User>,
  pub comments: usize,
  pub review_comments: usize,
  pub maintainer_can_modify: bool,
  pub commits: usize,
  pub additions: usize,
  pub deletions: usize,
  pub changed_files: usize,
}
// ts interface
pub struct Link {
  pub href: String,
}
// ts interface
pub struct PullRequestAutoMerge {
  pub enabled_by: User,
  pub merge_method: String,
  pub commit_title: String,
  pub commit_message: String,
}
// ts interface
pub struct PullRequestAutoMergeDisabledEvent {
  // pub action: UnknwonLiteral,
  pub number: usize,
  pub pull_request: PullRequest,
  pub reason: String,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct PullRequestAutoMergeEnabledEvent {
  // pub action: UnknwonLiteral,
  pub number: usize,
  pub pull_request: PullRequest,
  pub reason: String,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct PullRequestClosedEvent {
  // pub action: UnknwonLiteral,
  pub number: usize,
  // pub pull_request: UnknwonIntersection,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct PullRequestConvertedToDraftEvent {
  // pub action: UnknwonLiteral,
  pub number: usize,
  // pub pull_request: UnknwonIntersection,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct PullRequestDequeuedEvent {
  // pub action: UnknwonLiteral,
  pub number: usize,
  pub reason: String,
  pub pull_request: PullRequest,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct PullRequestEditedEvent {
  // pub action: UnknwonLiteral,
  pub number: usize,
  // pub changes: Unknwon,
  pub pull_request: PullRequest,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct PullRequestLabeledEvent {
  // pub action: UnknwonLiteral,
  pub number: usize,
  pub pull_request: PullRequest,
  pub label: Label,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct PullRequestLockedEvent {
  // pub action: UnknwonLiteral,
  pub number: usize,
  pub pull_request: PullRequest,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct PullRequestOpenedEvent {
  // pub action: UnknwonLiteral,
  pub number: usize,
  // pub pull_request: UnknwonIntersection,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct PullRequestQueuedEvent {
  // pub action: UnknwonLiteral,
  pub number: usize,
  pub pull_request: PullRequest,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct PullRequestReadyForReviewEvent {
  // pub action: UnknwonLiteral,
  pub number: usize,
  // pub pull_request: UnknwonIntersection,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct PullRequestReopenedEvent {
  // pub action: UnknwonLiteral,
  pub number: usize,
  // pub pull_request: UnknwonIntersection,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct PullRequestSynchronizeEvent {
  // pub action: UnknwonLiteral,
  pub number: usize,
  pub before: String,
  pub after: String,
  pub pull_request: PullRequest,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct PullRequestUnassignedEvent {
  // pub action: UnknwonLiteral,
  pub number: usize,
  pub pull_request: PullRequest,
  pub assignee: User,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct PullRequestUnlabeledEvent {
  // pub action: UnknwonLiteral,
  pub number: usize,
  pub pull_request: PullRequest,
  pub label: Label,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct PullRequestUnlockedEvent {
  // pub action: UnknwonLiteral,
  pub number: usize,
  pub pull_request: PullRequest,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct PullRequestReviewDismissedEvent {
  // pub action: UnknwonLiteral,
  // pub review: UnknwonIntersection,
  pub pull_request: SimplePullRequest,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct PullRequestReview {
  pub id: usize,
  pub node_id: String,
  pub user: User,
  pub body: Option<String>,
  pub commit_id: String,
  pub submitted_at: Option<String>,
  pub state: String,
  pub html_url: String,
  pub pull_request_url: String,
  pub author_association: AuthorAssociation,
  // pub _links: Unknwon,
}
// ts interface
pub struct SimplePullRequest {
  pub url: String,
  pub id: usize,
  pub node_id: String,
  pub html_url: String,
  pub diff_url: String,
  pub patch_url: String,
  pub issue_url: String,
  pub number: usize,
  pub state: String,
  pub locked: bool,
  pub title: String,
  pub user: User,
  pub body: Option<String>,
  pub created_at: String,
  pub updated_at: String,
  pub closed_at: Option<String>,
  pub merged_at: Option<String>,
  pub merge_commit_sha: Option<String>,
  pub assignee: Option<User>,
  pub assignees: Vec<User>,
  // pub requested_reviewers: Vec<Unknwon>,
  pub requested_teams: Vec<Team>,
  pub labels: Vec<Label>,
  pub milestone: Option<Milestone>,
  pub draft: bool,
  pub commits_url: String,
  pub review_comments_url: String,
  pub review_comment_url: String,
  pub comments_url: String,
  pub statuses_url: String,
  // pub head: Unknwon,
  // pub base: Unknwon,
  // pub _links: Unknwon,
  pub author_association: AuthorAssociation,
  pub auto_merge: Option<PullRequestAutoMerge>,
  pub active_lock_reason: Option<String>,
}
// ts interface
pub struct PullRequestReviewEditedEvent {
  // pub action: UnknwonLiteral,
  // pub changes: Unknwon,
  pub review: PullRequestReview,
  pub pull_request: SimplePullRequest,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct PullRequestReviewSubmittedEvent {
  // pub action: UnknwonLiteral,
  pub review: PullRequestReview,
  pub pull_request: SimplePullRequest,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct PullRequestReviewCommentCreatedEvent {
  // pub action: UnknwonLiteral,
  pub comment: PullRequestReviewComment,
  // pub pull_request: Unknwon,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct PullRequestReviewComment {
  pub url: String,
  pub pull_request_review_id: usize,
  pub id: usize,
  pub node_id: String,
  pub diff_hunk: String,
  pub path: String,
  pub position: Option<usize>,
  pub original_position: usize,
  pub commit_id: String,
  pub original_commit_id: String,
  pub user: User,
  pub body: String,
  pub created_at: String,
  pub updated_at: String,
  pub html_url: String,
  pub pull_request_url: String,
  pub author_association: AuthorAssociation,
  // pub _links: Unknwon,
  pub reactions: Reactions,
  pub start_line: Option<usize>,
  pub original_start_line: Option<usize>,
  pub start_side: Option<String>,
  pub line: Option<usize>,
  pub original_line: usize,
  pub side: String,
}
// ts interface
pub struct PullRequestReviewCommentDeletedEvent {
  // pub action: UnknwonLiteral,
  pub comment: PullRequestReviewComment,
  // pub pull_request: Unknwon,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct PullRequestReviewCommentEditedEvent {
  // pub action: UnknwonLiteral,
  // pub changes: Unknwon,
  pub comment: PullRequestReviewComment,
  // pub pull_request: Unknwon,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct PullRequestReviewThreadResolvedEvent {
  // pub action: UnknwonLiteral,
  // pub thread: Unknwon,
  pub pull_request: SimplePullRequest,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct PullRequestReviewThreadUnresolvedEvent {
  // pub action: UnknwonLiteral,
  // pub thread: Unknwon,
  pub pull_request: SimplePullRequest,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct PushEvent {
  pub ref_: String,
  pub before: String,
  pub after: String,
  pub created: bool,
  pub deleted: bool,
  pub forced: bool,
  pub base_ref: Option<String>,
  pub compare: String,
  pub commits: Vec<Commit>,
  pub head_commit: Option<Commit>,
  pub repository: Repository,
  pub pusher: Committer,
  pub sender: User,
}
// ts interface
pub struct Commit {
  pub id: String,
  pub tree_id: String,
  pub distinct: bool,
  pub message: String,
  pub timestamp: String,
  pub url: String,
  pub author: Committer,
  pub committer: Committer,
  pub added: Vec<String>,
  pub modified: Vec<String>,
  pub removed: Vec<String>,
}
// ts interface
pub struct RegistryPackagePublishedEvent {
  // pub action: UnknwonLiteral,
  // pub registry_package: Unknwon,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct RegistryPackageUpdatedEvent {
  // pub action: UnknwonLiteral,
  // pub registry_package: Unknwon,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct ReleaseCreatedEvent {
  // pub action: UnknwonLiteral,
  pub release: Release,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct Release {
  pub url: String,
  pub assets_url: String,
  pub upload_url: String,
  pub html_url: String,
  pub id: usize,
  pub node_id: String,
  pub tag_name: String,
  pub target_commitish: String,
  pub name: String,
  pub draft: bool,
  pub author: User,
  pub prerelease: bool,
  pub created_at: Option<String>,
  pub published_at: Option<String>,
  pub assets: Vec<ReleaseAsset>,
  pub tarball_url: Option<String>,
  pub zipball_url: Option<String>,
  pub body: String,
}
// ts interface
pub struct ReleaseAsset {
  pub url: String,
  pub browser_download_url: String,
  pub id: usize,
  pub node_id: String,
  pub name: String,
  pub label: Option<String>,
  // pub state: UnknwonLiteral,
  pub content_type: String,
  pub size: usize,
  pub download_count: usize,
  pub created_at: String,
  pub updated_at: String,
}
// ts interface
pub struct ReleaseDeletedEvent {
  // pub action: UnknwonLiteral,
  pub release: Release,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct ReleaseEditedEvent {
  // pub action: UnknwonLiteral,
  // pub changes: Unknwon,
  pub release: Release,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct ReleasePrereleasedEvent {
  // pub action: UnknwonLiteral,
  // pub release: UnknwonIntersection,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct ReleasePublishedEvent {
  // pub action: UnknwonLiteral,
  // pub release: UnknwonIntersection,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct ReleaseReleasedEvent {
  // pub action: UnknwonLiteral,
  pub release: Release,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct ReleaseUnpublishedEvent {
  // pub action: UnknwonLiteral,
  // pub release: UnknwonIntersection,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct RepositoryArchivedEvent {
  // pub action: UnknwonLiteral,
  // pub repository: UnknwonIntersection,
  pub sender: User,
}
// ts interface
pub struct RepositoryCreatedEvent {
  // pub action: UnknwonLiteral,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct RepositoryDeletedEvent {
  // pub action: UnknwonLiteral,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct RepositoryEditedEvent {
  // pub action: UnknwonLiteral,
  // pub changes: Unknwon,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct RepositoryPrivatizedEvent {
  // pub action: UnknwonLiteral,
  // pub repository: UnknwonIntersection,
  pub sender: User,
}
// ts interface
pub struct RepositoryPublicizedEvent {
  // pub action: UnknwonLiteral,
  // pub repository: UnknwonIntersection,
  pub sender: User,
}
// ts interface
pub struct RepositoryRenamedEvent {
  // pub action: UnknwonLiteral,
  // pub changes: Unknwon,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct RepositoryTransferredEvent {
  // pub action: UnknwonLiteral,
  // pub changes: Unknwon,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct RepositoryUnarchivedEvent {
  // pub action: UnknwonLiteral,
  // pub repository: UnknwonIntersection,
  pub sender: User,
}
// ts interface
pub struct RepositoryDispatchEvent {
  pub action: String,
  pub branch: String,
  // pub client_payload: Unknwon,
  pub repository: Repository,
  pub sender: User,
  pub installation: InstallationLite,
}
// ts interface
pub struct RepositoryImportEvent {
  pub status: String,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct RepositoryVulnerabilityAlertCreateEvent {
  // pub action: UnknwonLiteral,
  // pub alert: UnknwonIntersection,
  pub repository: Repository,
  pub sender: GitHubOrg,
}
// ts interface
pub struct RepositoryVulnerabilityAlertAlert {
  pub id: usize,
  pub number: usize,
  pub node_id: String,
  pub state: String,
  pub affected_range: String,
  pub affected_package_name: String,
  pub severity: String,
  pub ghsa_id: String,
  pub external_reference: String,
  pub external_identifier: String,
  pub fixed_in: String,
  pub created_at: String,
}
// ts interface
pub struct RepositoryVulnerabilityAlertDismissEvent {
  // pub action: UnknwonLiteral,
  // pub alert: UnknwonIntersection,
  pub repository: Repository,
  pub sender: GitHubOrg,
}
// ts interface
pub struct RepositoryVulnerabilityAlertReopenEvent {
  // pub action: UnknwonLiteral,
  // pub alert: UnknwonIntersection,
  pub repository: Repository,
  pub sender: GitHubOrg,
}
// ts interface
pub struct RepositoryVulnerabilityAlertResolveEvent {
  // pub action: UnknwonLiteral,
  // pub alert: UnknwonIntersection,
  pub repository: Repository,
  pub sender: GitHubOrg,
}
// ts interface
pub struct SecretScanningAlertCreatedEvent {
  // pub action: UnknwonLiteral,
  // pub alert: Unknwon,
  pub repository: Repository,
}
// ts interface
pub struct SecretScanningAlertReopenedEvent {
  // pub action: UnknwonLiteral,
  // pub alert: Unknwon,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct SecretScanningAlertResolvedEvent {
  // pub action: UnknwonLiteral,
  // pub alert: Unknwon,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct SecurityAdvisoryPerformedEvent {
  // pub action: UnknwonLiteral,
  // pub security_advisory: Unknwon,
}
// ts interface
pub struct SecurityAdvisoryPublishedEvent {
  // pub action: UnknwonLiteral,
  // pub security_advisory: Unknwon,
}
// ts interface
pub struct SecurityAdvisoryUpdatedEvent {
  // pub action: UnknwonLiteral,
  // pub security_advisory: Unknwon,
}
// ts interface
pub struct SecurityAdvisoryWithdrawnEvent {
  // pub action: UnknwonLiteral,
  // pub security_advisory: Unknwon,
}
// ts interface
pub struct SponsorshipCancelledEvent {
  // pub action: UnknwonLiteral,
  // pub sponsorship: Unknwon,
  pub sender: User,
}
// ts interface
pub struct SponsorshipTier {
  pub node_id: String,
  pub created_at: String,
  pub description: String,
  pub monthly_price_in_cents: usize,
  pub monthly_price_in_dollars: usize,
  pub name: String,
  pub is_one_time: bool,
  pub is_custom_ammount: bool,
}
// ts interface
pub struct SponsorshipCreatedEvent {
  // pub action: UnknwonLiteral,
  // pub sponsorship: Unknwon,
  pub sender: User,
}
// ts interface
pub struct SponsorshipEditedEvent {
  // pub action: UnknwonLiteral,
  // pub sponsorship: Unknwon,
  // pub changes: Unknwon,
  pub sender: User,
}
// ts interface
pub struct SponsorshipPendingCancellationEvent {
  // pub action: UnknwonLiteral,
  // pub sponsorship: Unknwon,
  pub sender: User,
}
// ts interface
pub struct SponsorshipPendingTierChangeEvent {
  // pub action: UnknwonLiteral,
  // pub sponsorship: Unknwon,
  // pub changes: Unknwon,
  pub sender: User,
}
// ts interface
pub struct SponsorshipTierChangedEvent {
  // pub action: UnknwonLiteral,
  // pub sponsorship: Unknwon,
  // pub changes: Unknwon,
  pub sender: User,
}
// ts interface
pub struct StarCreatedEvent {
  // pub action: UnknwonLiteral,
  pub starred_at: String,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct StarDeletedEvent {
  // pub action: UnknwonLiteral,
  pub starred_at: Option<()>,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct StatusEvent {
  pub id: usize,
  pub sha: String,
  pub name: String,
  pub target_url: Option<String>,
  pub context: String,
  pub description: Option<String>,
  pub state: String,
  // pub commit: Unknwon,
  // pub branches: Vec<Unknwon>,
  pub created_at: String,
  pub updated_at: String,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct TeamAddedToRepositoryEvent {
  // pub action: UnknwonLiteral,
  pub team: Team,
  pub sender: User,
  pub organization: Organization,
}
// ts interface
pub struct TeamCreatedEvent {
  // pub action: UnknwonLiteral,
  pub team: Team,
  pub sender: User,
  pub organization: Organization,
}
// ts interface
pub struct TeamDeletedEvent {
  // pub action: UnknwonLiteral,
  pub team: Team,
  pub sender: User,
  pub organization: Organization,
}
// ts interface
pub struct TeamEditedEvent {
  // pub action: UnknwonLiteral,
  // pub changes: Unknwon,
  pub team: Team,
  pub sender: User,
  pub organization: Organization,
}
// ts interface
pub struct TeamRemovedFromRepositoryEvent {
  // pub action: UnknwonLiteral,
  pub team: Team,
  pub sender: User,
  pub organization: Organization,
}
// ts interface
pub struct TeamAddEvent {
  pub team: Team,
  pub repository: Repository,
  pub sender: User,
  pub organization: Organization,
}
// ts interface
pub struct WatchStartedEvent {
  // pub action: UnknwonLiteral,
  pub repository: Repository,
  pub sender: User,
}
// ts interface
pub struct WorkflowDispatchEvent {
  // pub inputs: Option<Unknwon>,
  pub ref_: String,
  pub repository: Repository,
  pub sender: User,
  pub workflow: String,
}
// ts interface
pub struct WorkflowJobCompletedEvent {
  // pub action: UnknwonLiteral,
  pub repository: Repository,
  pub sender: User,
  // pub workflow_job: UnknwonIntersection,
}
// ts interface
pub struct WorkflowJob {
  pub id: usize,
  pub run_id: usize,
  pub run_attempt: usize,
  pub run_url: String,
  pub head_sha: String,
  pub node_id: String,
  pub name: String,
  pub check_run_url: String,
  pub html_url: String,
  pub url: String,
  pub status: String,
  pub steps: Vec<WorkflowStep>,
  pub conclusion: Option<String>,
  pub labels: Vec<String>,
  pub runner_id: Option<usize>,
  pub runner_name: Option<String>,
  pub runner_group_id: Option<usize>,
  pub runner_group_name: Option<String>,
  pub started_at: String,
  pub completed_at: Option<String>,
}
// ts interface
pub struct WorkflowStepInProgress {
  pub name: String,
  // pub status: UnknwonLiteral,
  pub conclusion: Option<()>,
  pub number: usize,
  pub started_at: String,
  pub completed_at: Option<()>,
}
// ts interface
pub struct WorkflowStepCompleted {
  pub name: String,
  // pub status: UnknwonLiteral,
  pub conclusion: String,
  pub number: usize,
  pub started_at: String,
  pub completed_at: String,
}
// ts interface
pub struct WorkflowJobInProgressEvent {
  // pub action: UnknwonLiteral,
  pub repository: Repository,
  pub sender: User,
  // pub workflow_job: UnknwonIntersection,
}
// ts interface
pub struct WorkflowJobQueuedEvent {
  // pub action: UnknwonLiteral,
  pub repository: Repository,
  pub sender: User,
  // pub workflow_job: UnknwonIntersection,
}
// ts interface
pub struct WorkflowRunCompletedEvent {
  // pub action: UnknwonLiteral,
  pub repository: Repository,
  pub sender: User,
  pub workflow: Workflow,
  // pub workflow_run: UnknwonIntersection,
}
// ts interface
pub struct WorkflowRun {
  pub artifacts_url: String,
  pub cancel_url: String,
  pub check_suite_url: String,
  pub check_suite_id: usize,
  pub check_suite_node_id: String,
  pub conclusion: Option<String>,
  pub created_at: String,
  pub event: String,
  pub head_branch: String,
  pub head_commit: SimpleCommit,
  pub head_repository: RepositoryLite,
  pub head_sha: String,
  pub path: String,
  pub display_title: String,
  pub html_url: String,
  pub id: usize,
  pub jobs_url: String,
  pub logs_url: String,
  pub node_id: String,
  pub name: String,
  // pub pull_requests: Vec<Unknwon>,
  pub repository: RepositoryLite,
  pub rerun_url: String,
  pub run_number: usize,
  pub status: String,
  pub updated_at: String,
  pub url: String,
  pub workflow_id: usize,
  pub workflow_url: String,
  pub run_attempt: usize,
  pub run_started_at: String,
  pub previous_attempt_url: Option<String>,
  pub actor: User,
  pub triggering_actor: User,
}
// ts interface
pub struct RepositoryLite {
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
  pub description: Option<String>,
  pub downloads_url: String,
  pub events_url: String,
  pub fork: bool,
  pub forks_url: String,
  pub full_name: String,
  pub git_commits_url: String,
  pub git_refs_url: String,
  pub git_tags_url: String,
  pub hooks_url: String,
  pub html_url: String,
  pub id: usize,
  pub issue_comment_url: String,
  pub issue_events_url: String,
  pub issues_url: String,
  pub keys_url: String,
  pub labels_url: String,
  pub languages_url: String,
  pub merges_url: String,
  pub milestones_url: String,
  pub name: String,
  pub node_id: String,
  pub notifications_url: String,
  pub owner: User,
  pub private: bool,
  pub pulls_url: String,
  pub releases_url: String,
  pub stargazers_url: String,
  pub statuses_url: String,
  pub subscribers_url: String,
  pub subscription_url: String,
  pub tags_url: String,
  pub teams_url: String,
  pub trees_url: String,
  pub url: String,
}
// ts interface
pub struct WorkflowRunInProgressEvent {
  // pub action: UnknwonLiteral,
  pub repository: Repository,
  pub sender: User,
  pub workflow: Workflow,
  pub workflow_run: WorkflowRun,
}
// ts interface
pub struct WorkflowRunRequestedEvent {
  // pub action: UnknwonLiteral,
  pub repository: Repository,
  pub sender: User,
  pub workflow: Workflow,
  pub workflow_run: WorkflowRun,
}
// ts interface
pub struct EventPayloadMap {
  pub branch_protection_rule: BranchProtectionRuleEvent,
  pub check_run: CheckRunEvent,
  pub check_suite: CheckSuiteEvent,
  pub code_scanning_alert: CodeScanningAlertEvent,
  pub commit_comment: CommitCommentEvent,
  pub create: CreateEvent,
  pub delete: DeleteEvent,
  pub dependabot_alert: DependabotAlertEvent,
  pub deploy_key: DeployKeyEvent,
  pub deployment: DeploymentEvent,
  pub deployment_status: DeploymentStatusEvent,
  pub discussion: DiscussionEvent,
  pub discussion_comment: DiscussionCommentEvent,
  pub fork: ForkEvent,
  pub github_app_authorization: GithubAppAuthorizationEvent,
  pub gollum: GollumEvent,
  pub installation: InstallationEvent,
  pub installation_repositories: InstallationRepositoriesEvent,
  pub issue_comment: IssueCommentEvent,
  pub issues: IssuesEvent,
  pub label: LabelEvent,
  pub marketplace_purchase: MarketplacePurchaseEvent,
  pub member: MemberEvent,
  pub membership: MembershipEvent,
  pub merge_group: MergeGroupEvent,
  pub meta: MetaEvent,
  pub milestone: MilestoneEvent,
  pub org_block: OrgBlockEvent,
  pub organization: OrganizationEvent,
  pub package: PackageEvent,
  pub page_build: PageBuildEvent,
  pub ping: PingEvent,
  pub project: ProjectEvent,
  pub project_card: ProjectCardEvent,
  pub project_column: ProjectColumnEvent,
  pub projects_v2_item: ProjectsV2ItemEvent,
  pub public: PublicEvent,
  pub pull_request: PullRequestEvent,
  pub pull_request_review: PullRequestReviewEvent,
  pub pull_request_review_comment: PullRequestReviewCommentEvent,
  pub pull_request_review_thread: PullRequestReviewThreadEvent,
  pub push: PushEvent,
  pub registry_package: RegistryPackageEvent,
  pub release: ReleaseEvent,
  pub repository: RepositoryEvent,
  pub repository_dispatch: RepositoryDispatchEvent,
  pub repository_import: RepositoryImportEvent,
  pub repository_vulnerability_alert: RepositoryVulnerabilityAlertEvent,
  pub secret_scanning_alert: SecretScanningAlertEvent,
  pub security_advisory: SecurityAdvisoryEvent,
  pub sponsorship: SponsorshipEvent,
  pub star: StarEvent,
  pub status: StatusEvent,
  pub team: TeamEvent,
  pub team_add: TeamAddEvent,
  pub watch: WatchEvent,
  pub workflow_dispatch: WorkflowDispatchEvent,
  pub workflow_job: WorkflowJobEvent,
  pub workflow_run: WorkflowRunEvent,
}
// ts type alias
pub type Asset = ReleaseAsset;
// ts type alias
pub type WebhookEvent = Schema;
// ts type alias
pub type WebhookEventMap = EventPayloadMap;
// ts type alias
