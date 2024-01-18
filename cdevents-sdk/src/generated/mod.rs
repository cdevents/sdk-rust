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

#[derive(Debug,Clone,Serialize,Deserialize)]
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
                Ok(Self::ArtifactPackaged(variant))
            },
            ARTIFACT_PUBLISHED => {
                let variant: artifact_published::Content = serde_json::from_value(json)?;
                Ok(Self::ArtifactPublished(variant))
            },
            ARTIFACT_SIGNED => {
                let variant: artifact_signed::Content = serde_json::from_value(json)?;
                Ok(Self::ArtifactSigned(variant))
            },
            BRANCH_CREATED => {
                let variant: branch_created::Content = serde_json::from_value(json)?;
                Ok(Self::BranchCreated(variant))
            },
            BRANCH_DELETED => {
                let variant: branch_deleted::Content = serde_json::from_value(json)?;
                Ok(Self::BranchDeleted(variant))
            },
            BUILD_FINISHED => {
                let variant: build_finished::Content = serde_json::from_value(json)?;
                Ok(Self::BuildFinished(variant))
            },
            BUILD_QUEUED => {
                let variant: build_queued::Content = serde_json::from_value(json)?;
                Ok(Self::BuildQueued(variant))
            },
            BUILD_STARTED => {
                let variant: build_started::Content = serde_json::from_value(json)?;
                Ok(Self::BuildStarted(variant))
            },
            CHANGE_ABANDONED => {
                let variant: change_abandoned::Content = serde_json::from_value(json)?;
                Ok(Self::ChangeAbandoned(variant))
            },
            CHANGE_CREATED => {
                let variant: change_created::Content = serde_json::from_value(json)?;
                Ok(Self::ChangeCreated(variant))
            },
            CHANGE_MERGED => {
                let variant: change_merged::Content = serde_json::from_value(json)?;
                Ok(Self::ChangeMerged(variant))
            },
            CHANGE_REVIEWED => {
                let variant: change_reviewed::Content = serde_json::from_value(json)?;
                Ok(Self::ChangeReviewed(variant))
            },
            CHANGE_UPDATED => {
                let variant: change_updated::Content = serde_json::from_value(json)?;
                Ok(Self::ChangeUpdated(variant))
            },
            ENVIRONMENT_CREATED => {
                let variant: environment_created::Content = serde_json::from_value(json)?;
                Ok(Self::EnvironmentCreated(variant))
            },
            ENVIRONMENT_DELETED => {
                let variant: environment_deleted::Content = serde_json::from_value(json)?;
                Ok(Self::EnvironmentDeleted(variant))
            },
            ENVIRONMENT_MODIFIED => {
                let variant: environment_modified::Content = serde_json::from_value(json)?;
                Ok(Self::EnvironmentModified(variant))
            },
            INCIDENT_DETECTED => {
                let variant: incident_detected::Content = serde_json::from_value(json)?;
                Ok(Self::IncidentDetected(variant))
            },
            INCIDENT_REPORTED => {
                let variant: incident_reported::Content = serde_json::from_value(json)?;
                Ok(Self::IncidentReported(variant))
            },
            INCIDENT_RESOLVED => {
                let variant: incident_resolved::Content = serde_json::from_value(json)?;
                Ok(Self::IncidentResolved(variant))
            },
            PIPELINERUN_FINISHED => {
                let variant: pipelinerun_finished::Content = serde_json::from_value(json)?;
                Ok(Self::PipelinerunFinished(variant))
            },
            PIPELINERUN_QUEUED => {
                let variant: pipelinerun_queued::Content = serde_json::from_value(json)?;
                Ok(Self::PipelinerunQueued(variant))
            },
            PIPELINERUN_STARTED => {
                let variant: pipelinerun_started::Content = serde_json::from_value(json)?;
                Ok(Self::PipelinerunStarted(variant))
            },
            REPOSITORY_CREATED => {
                let variant: repository_created::Content = serde_json::from_value(json)?;
                Ok(Self::RepositoryCreated(variant))
            },
            REPOSITORY_DELETED => {
                let variant: repository_deleted::Content = serde_json::from_value(json)?;
                Ok(Self::RepositoryDeleted(variant))
            },
            REPOSITORY_MODIFIED => {
                let variant: repository_modified::Content = serde_json::from_value(json)?;
                Ok(Self::RepositoryModified(variant))
            },
            SERVICE_DEPLOYED => {
                let variant: service_deployed::Content = serde_json::from_value(json)?;
                Ok(Self::ServiceDeployed(variant))
            },
            SERVICE_PUBLISHED => {
                let variant: service_published::Content = serde_json::from_value(json)?;
                Ok(Self::ServicePublished(variant))
            },
            SERVICE_REMOVED => {
                let variant: service_removed::Content = serde_json::from_value(json)?;
                Ok(Self::ServiceRemoved(variant))
            },
            SERVICE_ROLLEDBACK => {
                let variant: service_rolledback::Content = serde_json::from_value(json)?;
                Ok(Self::ServiceRolledback(variant))
            },
            SERVICE_UPGRADED => {
                let variant: service_upgraded::Content = serde_json::from_value(json)?;
                Ok(Self::ServiceUpgraded(variant))
            },
            TASKRUN_FINISHED => {
                let variant: taskrun_finished::Content = serde_json::from_value(json)?;
                Ok(Self::TaskrunFinished(variant))
            },
            TASKRUN_STARTED => {
                let variant: taskrun_started::Content = serde_json::from_value(json)?;
                Ok(Self::TaskrunStarted(variant))
            },
            TESTCASERUN_FINISHED => {
                let variant: testcaserun_finished::Content = serde_json::from_value(json)?;
                Ok(Self::TestcaserunFinished(variant))
            },
            TESTCASERUN_QUEUED => {
                let variant: testcaserun_queued::Content = serde_json::from_value(json)?;
                Ok(Self::TestcaserunQueued(variant))
            },
            TESTCASERUN_STARTED => {
                let variant: testcaserun_started::Content = serde_json::from_value(json)?;
                Ok(Self::TestcaserunStarted(variant))
            },
            TESTOUTPUT_PUBLISHED => {
                let variant: testoutput_published::Content = serde_json::from_value(json)?;
                Ok(Self::TestoutputPublished(variant))
            },
            TESTSUITERUN_FINISHED => {
                let variant: testsuiterun_finished::Content = serde_json::from_value(json)?;
                Ok(Self::TestsuiterunFinished(variant))
            },
            TESTSUITERUN_QUEUED => {
                let variant: testsuiterun_queued::Content = serde_json::from_value(json)?;
                Ok(Self::TestsuiterunQueued(variant))
            },
            TESTSUITERUN_STARTED => {
                let variant: testsuiterun_started::Content = serde_json::from_value(json)?;
                Ok(Self::TestsuiterunStarted(variant))
            },
            variant => Err(serde_json::Error::custom(format_args!(
                "unknown variant `{}`, expected 'dev.cdevents.{{subject}}.{{predicate}}.{{version}}'",
                variant,
            ))),
        }
    }
}
