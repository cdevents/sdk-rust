// @generated
// by cdevents/sdk-rust/generator (mod.hbs)

use serde::de::Error;

pub mod artifact_packaged;
pub mod artifact_published;
pub mod artifact_signed;
pub mod branch_created;
pub mod branch_deleted;
pub mod build_finished;
pub mod build_queued;
pub mod build_started;
pub mod change_abandoned;
pub mod change_created;
pub mod change_merged;
pub mod change_reviewed;
pub mod change_updated;
pub mod environment_created;
pub mod environment_deleted;
pub mod environment_modified;
pub mod incident_detected;
pub mod incident_reported;
pub mod incident_resolved;
pub mod pipelinerun_finished;
pub mod pipelinerun_queued;
pub mod pipelinerun_started;
pub mod repository_created;
pub mod repository_deleted;
pub mod repository_modified;
pub mod service_deployed;
pub mod service_published;
pub mod service_removed;
pub mod service_rolledback;
pub mod service_upgraded;
pub mod taskrun_finished;
pub mod taskrun_started;
pub mod testcaserun_finished;
pub mod testcaserun_queued;
pub mod testcaserun_started;
pub mod testoutput_published;
pub mod testsuiterun_finished;
pub mod testsuiterun_queued;
pub mod testsuiterun_started;

use serde::{Serialize, Deserialize};

pub const ARTIFACT_PACKAGED: &str = "dev.cdevents.artifact.packaged.0.1.1";
pub const ARTIFACT_PUBLISHED: &str = "dev.cdevents.artifact.published.0.1.1";
pub const ARTIFACT_SIGNED: &str = "dev.cdevents.artifact.signed.0.1.0";
pub const BRANCH_CREATED: &str = "dev.cdevents.branch.created.0.1.2";
pub const BRANCH_DELETED: &str = "dev.cdevents.branch.deleted.0.1.2";
pub const BUILD_FINISHED: &str = "dev.cdevents.build.finished.0.1.1";
pub const BUILD_QUEUED: &str = "dev.cdevents.build.queued.0.1.1";
pub const BUILD_STARTED: &str = "dev.cdevents.build.started.0.1.1";
pub const CHANGE_ABANDONED: &str = "dev.cdevents.change.abandoned.0.1.2";
pub const CHANGE_CREATED: &str = "dev.cdevents.change.created.0.1.2";
pub const CHANGE_MERGED: &str = "dev.cdevents.change.merged.0.1.2";
pub const CHANGE_REVIEWED: &str = "dev.cdevents.change.reviewed.0.1.2";
pub const CHANGE_UPDATED: &str = "dev.cdevents.change.updated.0.1.2";
pub const ENVIRONMENT_CREATED: &str = "dev.cdevents.environment.created.0.1.1";
pub const ENVIRONMENT_DELETED: &str = "dev.cdevents.environment.deleted.0.1.1";
pub const ENVIRONMENT_MODIFIED: &str = "dev.cdevents.environment.modified.0.1.1";
pub const INCIDENT_DETECTED: &str = "dev.cdevents.incident.detected.0.1.0";
pub const INCIDENT_REPORTED: &str = "dev.cdevents.incident.reported.0.1.0";
pub const INCIDENT_RESOLVED: &str = "dev.cdevents.incident.resolved.0.1.0";
pub const PIPELINERUN_FINISHED: &str = "dev.cdevents.pipelinerun.finished.0.1.1";
pub const PIPELINERUN_QUEUED: &str = "dev.cdevents.pipelinerun.queued.0.1.1";
pub const PIPELINERUN_STARTED: &str = "dev.cdevents.pipelinerun.started.0.1.1";
pub const REPOSITORY_CREATED: &str = "dev.cdevents.repository.created.0.1.1";
pub const REPOSITORY_DELETED: &str = "dev.cdevents.repository.deleted.0.1.1";
pub const REPOSITORY_MODIFIED: &str = "dev.cdevents.repository.modified.0.1.1";
pub const SERVICE_DEPLOYED: &str = "dev.cdevents.service.deployed.0.1.1";
pub const SERVICE_PUBLISHED: &str = "dev.cdevents.service.published.0.1.1";
pub const SERVICE_REMOVED: &str = "dev.cdevents.service.removed.0.1.1";
pub const SERVICE_ROLLEDBACK: &str = "dev.cdevents.service.rolledback.0.1.1";
pub const SERVICE_UPGRADED: &str = "dev.cdevents.service.upgraded.0.1.1";
pub const TASKRUN_FINISHED: &str = "dev.cdevents.taskrun.finished.0.1.1";
pub const TASKRUN_STARTED: &str = "dev.cdevents.taskrun.started.0.1.1";
pub const TESTCASERUN_FINISHED: &str = "dev.cdevents.testcaserun.finished.0.1.0";
pub const TESTCASERUN_QUEUED: &str = "dev.cdevents.testcaserun.queued.0.1.0";
pub const TESTCASERUN_STARTED: &str = "dev.cdevents.testcaserun.started.0.1.0";
pub const TESTOUTPUT_PUBLISHED: &str = "dev.cdevents.testoutput.published.0.1.0";
pub const TESTSUITERUN_FINISHED: &str = "dev.cdevents.testsuiterun.finished.0.1.0";
pub const TESTSUITERUN_QUEUED: &str = "dev.cdevents.testsuiterun.queued.0.1.0";
pub const TESTSUITERUN_STARTED: &str = "dev.cdevents.testsuiterun.started.0.1.0";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)] // TODO how to use content of context.type as discriminator ?
pub enum Content {
    ArtifactPackaged(artifact_packaged::Content),
    ArtifactPublished(artifact_published::Content),
    ArtifactSigned(artifact_signed::Content),
    BranchCreated(branch_created::Content),
    BranchDeleted(branch_deleted::Content),
    BuildFinished(build_finished::Content),
    BuildQueued(build_queued::Content),
    BuildStarted(build_started::Content),
    ChangeAbandoned(change_abandoned::Content),
    ChangeCreated(change_created::Content),
    ChangeMerged(change_merged::Content),
    ChangeReviewed(change_reviewed::Content),
    ChangeUpdated(change_updated::Content),
    EnvironmentCreated(environment_created::Content),
    EnvironmentDeleted(environment_deleted::Content),
    EnvironmentModified(environment_modified::Content),
    IncidentDetected(incident_detected::Content),
    IncidentReported(incident_reported::Content),
    IncidentResolved(incident_resolved::Content),
    PipelinerunFinished(pipelinerun_finished::Content),
    PipelinerunQueued(pipelinerun_queued::Content),
    PipelinerunStarted(pipelinerun_started::Content),
    RepositoryCreated(repository_created::Content),
    RepositoryDeleted(repository_deleted::Content),
    RepositoryModified(repository_modified::Content),
    ServiceDeployed(service_deployed::Content),
    ServicePublished(service_published::Content),
    ServiceRemoved(service_removed::Content),
    ServiceRolledback(service_rolledback::Content),
    ServiceUpgraded(service_upgraded::Content),
    TaskrunFinished(taskrun_finished::Content),
    TaskrunStarted(taskrun_started::Content),
    TestcaserunFinished(testcaserun_finished::Content),
    TestcaserunQueued(testcaserun_queued::Content),
    TestcaserunStarted(testcaserun_started::Content),
    TestoutputPublished(testoutput_published::Content),
    TestsuiterunFinished(testsuiterun_finished::Content),
    TestsuiterunQueued(testsuiterun_queued::Content),
    TestsuiterunStarted(testsuiterun_started::Content),
}

impl Content {
    pub fn from_json(ty: &str, json: serde_json::Value) -> Result<Self, serde_json::Error>{
        match ty {
            ARTIFACT_PACKAGED => {
                let variant: artifact_packaged::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            ARTIFACT_PUBLISHED => {
                let variant: artifact_published::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            ARTIFACT_SIGNED => {
                let variant: artifact_signed::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            BRANCH_CREATED => {
                let variant: branch_created::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            BRANCH_DELETED => {
                let variant: branch_deleted::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            BUILD_FINISHED => {
                let variant: build_finished::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            BUILD_QUEUED => {
                let variant: build_queued::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            BUILD_STARTED => {
                let variant: build_started::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            CHANGE_ABANDONED => {
                let variant: change_abandoned::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            CHANGE_CREATED => {
                let variant: change_created::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            CHANGE_MERGED => {
                let variant: change_merged::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            CHANGE_REVIEWED => {
                let variant: change_reviewed::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            CHANGE_UPDATED => {
                let variant: change_updated::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            ENVIRONMENT_CREATED => {
                let variant: environment_created::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            ENVIRONMENT_DELETED => {
                let variant: environment_deleted::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            ENVIRONMENT_MODIFIED => {
                let variant: environment_modified::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            INCIDENT_DETECTED => {
                let variant: incident_detected::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            INCIDENT_REPORTED => {
                let variant: incident_reported::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            INCIDENT_RESOLVED => {
                let variant: incident_resolved::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            PIPELINERUN_FINISHED => {
                let variant: pipelinerun_finished::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            PIPELINERUN_QUEUED => {
                let variant: pipelinerun_queued::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            PIPELINERUN_STARTED => {
                let variant: pipelinerun_started::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            REPOSITORY_CREATED => {
                let variant: repository_created::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            REPOSITORY_DELETED => {
                let variant: repository_deleted::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            REPOSITORY_MODIFIED => {
                let variant: repository_modified::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            SERVICE_DEPLOYED => {
                let variant: service_deployed::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            SERVICE_PUBLISHED => {
                let variant: service_published::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            SERVICE_REMOVED => {
                let variant: service_removed::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            SERVICE_ROLLEDBACK => {
                let variant: service_rolledback::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            SERVICE_UPGRADED => {
                let variant: service_upgraded::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            TASKRUN_FINISHED => {
                let variant: taskrun_finished::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            TASKRUN_STARTED => {
                let variant: taskrun_started::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            TESTCASERUN_FINISHED => {
                let variant: testcaserun_finished::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            TESTCASERUN_QUEUED => {
                let variant: testcaserun_queued::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            TESTCASERUN_STARTED => {
                let variant: testcaserun_started::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            TESTOUTPUT_PUBLISHED => {
                let variant: testoutput_published::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            TESTSUITERUN_FINISHED => {
                let variant: testsuiterun_finished::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            TESTSUITERUN_QUEUED => {
                let variant: testsuiterun_queued::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            TESTSUITERUN_STARTED => {
                let variant: testsuiterun_started::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            variant => Err(serde_json::Error::custom(format_args!(
                "unknown variant `{}`, expected 'dev.cdevents.{{subject}}.{{predicate}}.{{version}}'",
                variant,
            ))),
        }
    }

    pub fn ty(&self) -> &'static str {
        match self {
            Self::ArtifactPackaged(_) => ARTIFACT_PACKAGED,
            Self::ArtifactPublished(_) => ARTIFACT_PUBLISHED,
            Self::ArtifactSigned(_) => ARTIFACT_SIGNED,
            Self::BranchCreated(_) => BRANCH_CREATED,
            Self::BranchDeleted(_) => BRANCH_DELETED,
            Self::BuildFinished(_) => BUILD_FINISHED,
            Self::BuildQueued(_) => BUILD_QUEUED,
            Self::BuildStarted(_) => BUILD_STARTED,
            Self::ChangeAbandoned(_) => CHANGE_ABANDONED,
            Self::ChangeCreated(_) => CHANGE_CREATED,
            Self::ChangeMerged(_) => CHANGE_MERGED,
            Self::ChangeReviewed(_) => CHANGE_REVIEWED,
            Self::ChangeUpdated(_) => CHANGE_UPDATED,
            Self::EnvironmentCreated(_) => ENVIRONMENT_CREATED,
            Self::EnvironmentDeleted(_) => ENVIRONMENT_DELETED,
            Self::EnvironmentModified(_) => ENVIRONMENT_MODIFIED,
            Self::IncidentDetected(_) => INCIDENT_DETECTED,
            Self::IncidentReported(_) => INCIDENT_REPORTED,
            Self::IncidentResolved(_) => INCIDENT_RESOLVED,
            Self::PipelinerunFinished(_) => PIPELINERUN_FINISHED,
            Self::PipelinerunQueued(_) => PIPELINERUN_QUEUED,
            Self::PipelinerunStarted(_) => PIPELINERUN_STARTED,
            Self::RepositoryCreated(_) => REPOSITORY_CREATED,
            Self::RepositoryDeleted(_) => REPOSITORY_DELETED,
            Self::RepositoryModified(_) => REPOSITORY_MODIFIED,
            Self::ServiceDeployed(_) => SERVICE_DEPLOYED,
            Self::ServicePublished(_) => SERVICE_PUBLISHED,
            Self::ServiceRemoved(_) => SERVICE_REMOVED,
            Self::ServiceRolledback(_) => SERVICE_ROLLEDBACK,
            Self::ServiceUpgraded(_) => SERVICE_UPGRADED,
            Self::TaskrunFinished(_) => TASKRUN_FINISHED,
            Self::TaskrunStarted(_) => TASKRUN_STARTED,
            Self::TestcaserunFinished(_) => TESTCASERUN_FINISHED,
            Self::TestcaserunQueued(_) => TESTCASERUN_QUEUED,
            Self::TestcaserunStarted(_) => TESTCASERUN_STARTED,
            Self::TestoutputPublished(_) => TESTOUTPUT_PUBLISHED,
            Self::TestsuiterunFinished(_) => TESTSUITERUN_FINISHED,
            Self::TestsuiterunQueued(_) => TESTSUITERUN_QUEUED,
            Self::TestsuiterunStarted(_) => TESTSUITERUN_STARTED,
        }
    }

    pub fn subject(&self) -> &'static str {
        match self {
            Self::ArtifactPackaged(_) => "artifact",
            Self::ArtifactPublished(_) => "artifact",
            Self::ArtifactSigned(_) => "artifact",
            Self::BranchCreated(_) => "branch",
            Self::BranchDeleted(_) => "branch",
            Self::BuildFinished(_) => "build",
            Self::BuildQueued(_) => "build",
            Self::BuildStarted(_) => "build",
            Self::ChangeAbandoned(_) => "change",
            Self::ChangeCreated(_) => "change",
            Self::ChangeMerged(_) => "change",
            Self::ChangeReviewed(_) => "change",
            Self::ChangeUpdated(_) => "change",
            Self::EnvironmentCreated(_) => "environment",
            Self::EnvironmentDeleted(_) => "environment",
            Self::EnvironmentModified(_) => "environment",
            Self::IncidentDetected(_) => "incident",
            Self::IncidentReported(_) => "incident",
            Self::IncidentResolved(_) => "incident",
            Self::PipelinerunFinished(_) => "pipelineRun",
            Self::PipelinerunQueued(_) => "pipelineRun",
            Self::PipelinerunStarted(_) => "pipelineRun",
            Self::RepositoryCreated(_) => "repository",
            Self::RepositoryDeleted(_) => "repository",
            Self::RepositoryModified(_) => "repository",
            Self::ServiceDeployed(_) => "service",
            Self::ServicePublished(_) => "service",
            Self::ServiceRemoved(_) => "service",
            Self::ServiceRolledback(_) => "service",
            Self::ServiceUpgraded(_) => "service",
            Self::TaskrunFinished(_) => "taskRun",
            Self::TaskrunStarted(_) => "taskRun",
            Self::TestcaserunFinished(_) => "testCaseRun",
            Self::TestcaserunQueued(_) => "testCaseRun",
            Self::TestcaserunStarted(_) => "testCaseRun",
            Self::TestoutputPublished(_) => "testOutput",
            Self::TestsuiterunFinished(_) => "testSuiteRun",
            Self::TestsuiterunQueued(_) => "testSuiteRun",
            Self::TestsuiterunStarted(_) => "testSuiteRun",
        }
    }

    pub fn predicate(&self) -> &'static str {
        match self {
            Self::ArtifactPackaged(_) => "packaged",
            Self::ArtifactPublished(_) => "published",
            Self::ArtifactSigned(_) => "signed",
            Self::BranchCreated(_) => "created",
            Self::BranchDeleted(_) => "deleted",
            Self::BuildFinished(_) => "finished",
            Self::BuildQueued(_) => "queued",
            Self::BuildStarted(_) => "started",
            Self::ChangeAbandoned(_) => "abandoned",
            Self::ChangeCreated(_) => "created",
            Self::ChangeMerged(_) => "merged",
            Self::ChangeReviewed(_) => "reviewed",
            Self::ChangeUpdated(_) => "updated",
            Self::EnvironmentCreated(_) => "created",
            Self::EnvironmentDeleted(_) => "deleted",
            Self::EnvironmentModified(_) => "modified",
            Self::IncidentDetected(_) => "detected",
            Self::IncidentReported(_) => "reported",
            Self::IncidentResolved(_) => "resolved",
            Self::PipelinerunFinished(_) => "finished",
            Self::PipelinerunQueued(_) => "queued",
            Self::PipelinerunStarted(_) => "started",
            Self::RepositoryCreated(_) => "created",
            Self::RepositoryDeleted(_) => "deleted",
            Self::RepositoryModified(_) => "modified",
            Self::ServiceDeployed(_) => "deployed",
            Self::ServicePublished(_) => "published",
            Self::ServiceRemoved(_) => "removed",
            Self::ServiceRolledback(_) => "rolledback",
            Self::ServiceUpgraded(_) => "upgraded",
            Self::TaskrunFinished(_) => "finished",
            Self::TaskrunStarted(_) => "started",
            Self::TestcaserunFinished(_) => "finished",
            Self::TestcaserunQueued(_) => "queued",
            Self::TestcaserunStarted(_) => "started",
            Self::TestoutputPublished(_) => "published",
            Self::TestsuiterunFinished(_) => "finished",
            Self::TestsuiterunQueued(_) => "queued",
            Self::TestsuiterunStarted(_) => "started",
        }
    }
}

// due to inconstency in case/format the subject could be not be extracted from the context.type (ty), jsonshema $id, spec filename (shema, examples)
pub fn extract_subject_predicate(ty: &str) -> Option<(&str, &str)>{
    // let mut split = ty.split('.');
    match ty {
        ARTIFACT_PACKAGED => Some(("artifact", "packaged")),
        ARTIFACT_PUBLISHED => Some(("artifact", "published")),
        ARTIFACT_SIGNED => Some(("artifact", "signed")),
        BRANCH_CREATED => Some(("branch", "created")),
        BRANCH_DELETED => Some(("branch", "deleted")),
        BUILD_FINISHED => Some(("build", "finished")),
        BUILD_QUEUED => Some(("build", "queued")),
        BUILD_STARTED => Some(("build", "started")),
        CHANGE_ABANDONED => Some(("change", "abandoned")),
        CHANGE_CREATED => Some(("change", "created")),
        CHANGE_MERGED => Some(("change", "merged")),
        CHANGE_REVIEWED => Some(("change", "reviewed")),
        CHANGE_UPDATED => Some(("change", "updated")),
        ENVIRONMENT_CREATED => Some(("environment", "created")),
        ENVIRONMENT_DELETED => Some(("environment", "deleted")),
        ENVIRONMENT_MODIFIED => Some(("environment", "modified")),
        INCIDENT_DETECTED => Some(("incident", "detected")),
        INCIDENT_REPORTED => Some(("incident", "reported")),
        INCIDENT_RESOLVED => Some(("incident", "resolved")),
        PIPELINERUN_FINISHED => Some(("pipelineRun", "finished")),
        PIPELINERUN_QUEUED => Some(("pipelineRun", "queued")),
        PIPELINERUN_STARTED => Some(("pipelineRun", "started")),
        REPOSITORY_CREATED => Some(("repository", "created")),
        REPOSITORY_DELETED => Some(("repository", "deleted")),
        REPOSITORY_MODIFIED => Some(("repository", "modified")),
        SERVICE_DEPLOYED => Some(("service", "deployed")),
        SERVICE_PUBLISHED => Some(("service", "published")),
        SERVICE_REMOVED => Some(("service", "removed")),
        SERVICE_ROLLEDBACK => Some(("service", "rolledback")),
        SERVICE_UPGRADED => Some(("service", "upgraded")),
        TASKRUN_FINISHED => Some(("taskRun", "finished")),
        TASKRUN_STARTED => Some(("taskRun", "started")),
        TESTCASERUN_FINISHED => Some(("testCaseRun", "finished")),
        TESTCASERUN_QUEUED => Some(("testCaseRun", "queued")),
        TESTCASERUN_STARTED => Some(("testCaseRun", "started")),
        TESTOUTPUT_PUBLISHED => Some(("testOutput", "published")),
        TESTSUITERUN_FINISHED => Some(("testSuiteRun", "finished")),
        TESTSUITERUN_QUEUED => Some(("testSuiteRun", "queued")),
        TESTSUITERUN_STARTED => Some(("testSuiteRun", "started")),
        _ => None,
    }
}

impl From<artifact_packaged::Content> for Content {
    fn from(value: artifact_packaged::Content) -> Self {
        Self::ArtifactPackaged(value)
    }
}
impl From<artifact_published::Content> for Content {
    fn from(value: artifact_published::Content) -> Self {
        Self::ArtifactPublished(value)
    }
}
impl From<artifact_signed::Content> for Content {
    fn from(value: artifact_signed::Content) -> Self {
        Self::ArtifactSigned(value)
    }
}
impl From<branch_created::Content> for Content {
    fn from(value: branch_created::Content) -> Self {
        Self::BranchCreated(value)
    }
}
impl From<branch_deleted::Content> for Content {
    fn from(value: branch_deleted::Content) -> Self {
        Self::BranchDeleted(value)
    }
}
impl From<build_finished::Content> for Content {
    fn from(value: build_finished::Content) -> Self {
        Self::BuildFinished(value)
    }
}
impl From<build_queued::Content> for Content {
    fn from(value: build_queued::Content) -> Self {
        Self::BuildQueued(value)
    }
}
impl From<build_started::Content> for Content {
    fn from(value: build_started::Content) -> Self {
        Self::BuildStarted(value)
    }
}
impl From<change_abandoned::Content> for Content {
    fn from(value: change_abandoned::Content) -> Self {
        Self::ChangeAbandoned(value)
    }
}
impl From<change_created::Content> for Content {
    fn from(value: change_created::Content) -> Self {
        Self::ChangeCreated(value)
    }
}
impl From<change_merged::Content> for Content {
    fn from(value: change_merged::Content) -> Self {
        Self::ChangeMerged(value)
    }
}
impl From<change_reviewed::Content> for Content {
    fn from(value: change_reviewed::Content) -> Self {
        Self::ChangeReviewed(value)
    }
}
impl From<change_updated::Content> for Content {
    fn from(value: change_updated::Content) -> Self {
        Self::ChangeUpdated(value)
    }
}
impl From<environment_created::Content> for Content {
    fn from(value: environment_created::Content) -> Self {
        Self::EnvironmentCreated(value)
    }
}
impl From<environment_deleted::Content> for Content {
    fn from(value: environment_deleted::Content) -> Self {
        Self::EnvironmentDeleted(value)
    }
}
impl From<environment_modified::Content> for Content {
    fn from(value: environment_modified::Content) -> Self {
        Self::EnvironmentModified(value)
    }
}
impl From<incident_detected::Content> for Content {
    fn from(value: incident_detected::Content) -> Self {
        Self::IncidentDetected(value)
    }
}
impl From<incident_reported::Content> for Content {
    fn from(value: incident_reported::Content) -> Self {
        Self::IncidentReported(value)
    }
}
impl From<incident_resolved::Content> for Content {
    fn from(value: incident_resolved::Content) -> Self {
        Self::IncidentResolved(value)
    }
}
impl From<pipelinerun_finished::Content> for Content {
    fn from(value: pipelinerun_finished::Content) -> Self {
        Self::PipelinerunFinished(value)
    }
}
impl From<pipelinerun_queued::Content> for Content {
    fn from(value: pipelinerun_queued::Content) -> Self {
        Self::PipelinerunQueued(value)
    }
}
impl From<pipelinerun_started::Content> for Content {
    fn from(value: pipelinerun_started::Content) -> Self {
        Self::PipelinerunStarted(value)
    }
}
impl From<repository_created::Content> for Content {
    fn from(value: repository_created::Content) -> Self {
        Self::RepositoryCreated(value)
    }
}
impl From<repository_deleted::Content> for Content {
    fn from(value: repository_deleted::Content) -> Self {
        Self::RepositoryDeleted(value)
    }
}
impl From<repository_modified::Content> for Content {
    fn from(value: repository_modified::Content) -> Self {
        Self::RepositoryModified(value)
    }
}
impl From<service_deployed::Content> for Content {
    fn from(value: service_deployed::Content) -> Self {
        Self::ServiceDeployed(value)
    }
}
impl From<service_published::Content> for Content {
    fn from(value: service_published::Content) -> Self {
        Self::ServicePublished(value)
    }
}
impl From<service_removed::Content> for Content {
    fn from(value: service_removed::Content) -> Self {
        Self::ServiceRemoved(value)
    }
}
impl From<service_rolledback::Content> for Content {
    fn from(value: service_rolledback::Content) -> Self {
        Self::ServiceRolledback(value)
    }
}
impl From<service_upgraded::Content> for Content {
    fn from(value: service_upgraded::Content) -> Self {
        Self::ServiceUpgraded(value)
    }
}
impl From<taskrun_finished::Content> for Content {
    fn from(value: taskrun_finished::Content) -> Self {
        Self::TaskrunFinished(value)
    }
}
impl From<taskrun_started::Content> for Content {
    fn from(value: taskrun_started::Content) -> Self {
        Self::TaskrunStarted(value)
    }
}
impl From<testcaserun_finished::Content> for Content {
    fn from(value: testcaserun_finished::Content) -> Self {
        Self::TestcaserunFinished(value)
    }
}
impl From<testcaserun_queued::Content> for Content {
    fn from(value: testcaserun_queued::Content) -> Self {
        Self::TestcaserunQueued(value)
    }
}
impl From<testcaserun_started::Content> for Content {
    fn from(value: testcaserun_started::Content) -> Self {
        Self::TestcaserunStarted(value)
    }
}
impl From<testoutput_published::Content> for Content {
    fn from(value: testoutput_published::Content) -> Self {
        Self::TestoutputPublished(value)
    }
}
impl From<testsuiterun_finished::Content> for Content {
    fn from(value: testsuiterun_finished::Content) -> Self {
        Self::TestsuiterunFinished(value)
    }
}
impl From<testsuiterun_queued::Content> for Content {
    fn from(value: testsuiterun_queued::Content) -> Self {
        Self::TestsuiterunQueued(value)
    }
}
impl From<testsuiterun_started::Content> for Content {
    fn from(value: testsuiterun_started::Content) -> Self {
        Self::TestsuiterunStarted(value)
    }
}

#[cfg(feature = "testkit")]
impl<> proptest::arbitrary::Arbitrary for Content {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        prop_oneof![
            any::<artifact_packaged::Content>().prop_map(Content::from),
            any::<artifact_published::Content>().prop_map(Content::from),
            any::<artifact_signed::Content>().prop_map(Content::from),
            any::<branch_created::Content>().prop_map(Content::from),
            any::<branch_deleted::Content>().prop_map(Content::from),
            any::<build_finished::Content>().prop_map(Content::from),
            any::<build_queued::Content>().prop_map(Content::from),
            any::<build_started::Content>().prop_map(Content::from),
            any::<change_abandoned::Content>().prop_map(Content::from),
            any::<change_created::Content>().prop_map(Content::from),
            any::<change_merged::Content>().prop_map(Content::from),
            any::<change_reviewed::Content>().prop_map(Content::from),
            any::<change_updated::Content>().prop_map(Content::from),
            any::<environment_created::Content>().prop_map(Content::from),
            any::<environment_deleted::Content>().prop_map(Content::from),
            any::<environment_modified::Content>().prop_map(Content::from),
            any::<incident_detected::Content>().prop_map(Content::from),
            any::<incident_reported::Content>().prop_map(Content::from),
            any::<incident_resolved::Content>().prop_map(Content::from),
            any::<pipelinerun_finished::Content>().prop_map(Content::from),
            any::<pipelinerun_queued::Content>().prop_map(Content::from),
            any::<pipelinerun_started::Content>().prop_map(Content::from),
            any::<repository_created::Content>().prop_map(Content::from),
            any::<repository_deleted::Content>().prop_map(Content::from),
            any::<repository_modified::Content>().prop_map(Content::from),
            any::<service_deployed::Content>().prop_map(Content::from),
            any::<service_published::Content>().prop_map(Content::from),
            any::<service_removed::Content>().prop_map(Content::from),
            any::<service_rolledback::Content>().prop_map(Content::from),
            any::<service_upgraded::Content>().prop_map(Content::from),
            any::<taskrun_finished::Content>().prop_map(Content::from),
            any::<taskrun_started::Content>().prop_map(Content::from),
            any::<testcaserun_finished::Content>().prop_map(Content::from),
            any::<testcaserun_queued::Content>().prop_map(Content::from),
            any::<testcaserun_started::Content>().prop_map(Content::from),
            any::<testoutput_published::Content>().prop_map(Content::from),
            any::<testsuiterun_finished::Content>().prop_map(Content::from),
            any::<testsuiterun_queued::Content>().prop_map(Content::from),
            any::<testsuiterun_started::Content>().prop_map(Content::from),
        ].boxed()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// 
//     #[test]
//     fn test_true() {
//         
//         assert_eq!(extract_subject_predicate(ARTIFACT_PACKAGED), Some(("artifact","packaged")));
//         
//         assert_eq!(extract_subject_predicate(ARTIFACT_PUBLISHED), Some(("artifact","published")));
//         
//         assert_eq!(extract_subject_predicate(ARTIFACT_SIGNED), Some(("artifact","signed")));
//         
//         assert_eq!(extract_subject_predicate(BRANCH_CREATED), Some(("branch","created")));
//         
//         assert_eq!(extract_subject_predicate(BRANCH_DELETED), Some(("branch","deleted")));
//         
//         assert_eq!(extract_subject_predicate(BUILD_FINISHED), Some(("build","finished")));
//         
//         assert_eq!(extract_subject_predicate(BUILD_QUEUED), Some(("build","queued")));
//         
//         assert_eq!(extract_subject_predicate(BUILD_STARTED), Some(("build","started")));
//         
//         assert_eq!(extract_subject_predicate(CHANGE_ABANDONED), Some(("change","abandoned")));
//         
//         assert_eq!(extract_subject_predicate(CHANGE_CREATED), Some(("change","created")));
//         
//         assert_eq!(extract_subject_predicate(CHANGE_MERGED), Some(("change","merged")));
//         
//         assert_eq!(extract_subject_predicate(CHANGE_REVIEWED), Some(("change","reviewed")));
//         
//         assert_eq!(extract_subject_predicate(CHANGE_UPDATED), Some(("change","updated")));
//         
//         assert_eq!(extract_subject_predicate(ENVIRONMENT_CREATED), Some(("environment","created")));
//         
//         assert_eq!(extract_subject_predicate(ENVIRONMENT_DELETED), Some(("environment","deleted")));
//         
//         assert_eq!(extract_subject_predicate(ENVIRONMENT_MODIFIED), Some(("environment","modified")));
//         
//         assert_eq!(extract_subject_predicate(INCIDENT_DETECTED), Some(("incident","detected")));
//         
//         assert_eq!(extract_subject_predicate(INCIDENT_REPORTED), Some(("incident","reported")));
//         
//         assert_eq!(extract_subject_predicate(INCIDENT_RESOLVED), Some(("incident","resolved")));
//         
//         assert_eq!(extract_subject_predicate(PIPELINERUN_FINISHED), Some(("pipelineRun","finished")));
//         
//         assert_eq!(extract_subject_predicate(PIPELINERUN_QUEUED), Some(("pipelineRun","queued")));
//         
//         assert_eq!(extract_subject_predicate(PIPELINERUN_STARTED), Some(("pipelineRun","started")));
//         
//         assert_eq!(extract_subject_predicate(REPOSITORY_CREATED), Some(("repository","created")));
//         
//         assert_eq!(extract_subject_predicate(REPOSITORY_DELETED), Some(("repository","deleted")));
//         
//         assert_eq!(extract_subject_predicate(REPOSITORY_MODIFIED), Some(("repository","modified")));
//         
//         assert_eq!(extract_subject_predicate(SERVICE_DEPLOYED), Some(("service","deployed")));
//         
//         assert_eq!(extract_subject_predicate(SERVICE_PUBLISHED), Some(("service","published")));
//         
//         assert_eq!(extract_subject_predicate(SERVICE_REMOVED), Some(("service","removed")));
//         
//         assert_eq!(extract_subject_predicate(SERVICE_ROLLEDBACK), Some(("service","rolledback")));
//         
//         assert_eq!(extract_subject_predicate(SERVICE_UPGRADED), Some(("service","upgraded")));
//         
//         assert_eq!(extract_subject_predicate(TASKRUN_FINISHED), Some(("taskRun","finished")));
//         
//         assert_eq!(extract_subject_predicate(TASKRUN_STARTED), Some(("taskRun","started")));
//         
//         assert_eq!(extract_subject_predicate(TESTCASERUN_FINISHED), Some(("testCaseRun","finished")));
//         
//         assert_eq!(extract_subject_predicate(TESTCASERUN_QUEUED), Some(("testCaseRun","queued")));
//         
//         assert_eq!(extract_subject_predicate(TESTCASERUN_STARTED), Some(("testCaseRun","started")));
//         
//         assert_eq!(extract_subject_predicate(TESTOUTPUT_PUBLISHED), Some(("testOutput","published")));
//         
//         assert_eq!(extract_subject_predicate(TESTSUITERUN_FINISHED), Some(("testSuiteRun","finished")));
//         
//         assert_eq!(extract_subject_predicate(TESTSUITERUN_QUEUED), Some(("testSuiteRun","queued")));
//         
//         assert_eq!(extract_subject_predicate(TESTSUITERUN_STARTED), Some(("testSuiteRun","started")));
//         
//     }
// }