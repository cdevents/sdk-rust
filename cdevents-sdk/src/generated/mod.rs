// @generated
// by cdevents/sdk-rust/generator (mod.hbs)

use serde::{Serialize, Deserialize, de::Error};

pub mod artifact_packaged_0_1_1;
pub mod artifact_published_0_1_1;
pub mod artifact_signed_0_1_0;
pub mod branch_created_0_1_2;
pub mod branch_deleted_0_1_2;
pub mod build_finished_0_1_1;
pub mod build_queued_0_1_1;
pub mod build_started_0_1_1;
pub mod change_abandoned_0_1_2;
pub mod change_created_0_2_0;
pub mod change_created_0_1_2;
pub mod change_merged_0_1_2;
pub mod change_reviewed_0_1_2;
pub mod change_updated_0_1_2;
pub mod environment_created_0_1_1;
pub mod environment_deleted_0_1_1;
pub mod environment_modified_0_1_1;
pub mod incident_detected_0_1_0;
pub mod incident_reported_0_1_0;
pub mod incident_resolved_0_1_0;
pub mod pipelinerun_finished_0_1_1;
pub mod pipelinerun_queued_0_1_1;
pub mod pipelinerun_started_0_1_1;
pub mod repository_created_0_1_1;
pub mod repository_deleted_0_1_1;
pub mod repository_modified_0_1_1;
pub mod service_deployed_0_1_1;
pub mod service_published_0_1_1;
pub mod service_removed_0_1_1;
pub mod service_rolledback_0_1_1;
pub mod service_upgraded_0_1_1;
pub mod taskrun_finished_0_1_1;
pub mod taskrun_started_0_1_1;
pub mod testcaserun_finished_0_1_0;
pub mod testcaserun_queued_0_1_0;
pub mod testcaserun_started_0_1_0;
pub mod testoutput_published_0_1_0;
pub mod testsuiterun_finished_0_1_0;
pub mod testsuiterun_queued_0_1_0;
pub mod testsuiterun_started_0_1_0;

pub mod latest {
    pub use super::artifact_packaged_0_1_1 as artifact_packaged;
    pub use super::artifact_published_0_1_1 as artifact_published;
    pub use super::artifact_signed_0_1_0 as artifact_signed;
    pub use super::branch_created_0_1_2 as branch_created;
    pub use super::branch_deleted_0_1_2 as branch_deleted;
    pub use super::build_finished_0_1_1 as build_finished;
    pub use super::build_queued_0_1_1 as build_queued;
    pub use super::build_started_0_1_1 as build_started;
    pub use super::change_abandoned_0_1_2 as change_abandoned;
    pub use super::change_created_0_2_0 as change_created;
    pub use super::change_merged_0_1_2 as change_merged;
    pub use super::change_reviewed_0_1_2 as change_reviewed;
    pub use super::change_updated_0_1_2 as change_updated;
    pub use super::environment_created_0_1_1 as environment_created;
    pub use super::environment_deleted_0_1_1 as environment_deleted;
    pub use super::environment_modified_0_1_1 as environment_modified;
    pub use super::incident_detected_0_1_0 as incident_detected;
    pub use super::incident_reported_0_1_0 as incident_reported;
    pub use super::incident_resolved_0_1_0 as incident_resolved;
    pub use super::pipelinerun_finished_0_1_1 as pipelinerun_finished;
    pub use super::pipelinerun_queued_0_1_1 as pipelinerun_queued;
    pub use super::pipelinerun_started_0_1_1 as pipelinerun_started;
    pub use super::repository_created_0_1_1 as repository_created;
    pub use super::repository_deleted_0_1_1 as repository_deleted;
    pub use super::repository_modified_0_1_1 as repository_modified;
    pub use super::service_deployed_0_1_1 as service_deployed;
    pub use super::service_published_0_1_1 as service_published;
    pub use super::service_removed_0_1_1 as service_removed;
    pub use super::service_rolledback_0_1_1 as service_rolledback;
    pub use super::service_upgraded_0_1_1 as service_upgraded;
    pub use super::taskrun_finished_0_1_1 as taskrun_finished;
    pub use super::taskrun_started_0_1_1 as taskrun_started;
    pub use super::testcaserun_finished_0_1_0 as testcaserun_finished;
    pub use super::testcaserun_queued_0_1_0 as testcaserun_queued;
    pub use super::testcaserun_started_0_1_0 as testcaserun_started;
    pub use super::testoutput_published_0_1_0 as testoutput_published;
    pub use super::testsuiterun_finished_0_1_0 as testsuiterun_finished;
    pub use super::testsuiterun_queued_0_1_0 as testsuiterun_queued;
    pub use super::testsuiterun_started_0_1_0 as testsuiterun_started;
}
pub mod spec_0_3_0 {
    pub use super::artifact_packaged_0_1_1 as artifact_packaged;
    pub use super::artifact_published_0_1_1 as artifact_published;
    pub use super::artifact_signed_0_1_0 as artifact_signed;
    pub use super::branch_created_0_1_2 as branch_created;
    pub use super::branch_deleted_0_1_2 as branch_deleted;
    pub use super::build_finished_0_1_1 as build_finished;
    pub use super::build_queued_0_1_1 as build_queued;
    pub use super::build_started_0_1_1 as build_started;
    pub use super::change_abandoned_0_1_2 as change_abandoned;
    pub use super::change_created_0_1_2 as change_created;
    pub use super::change_merged_0_1_2 as change_merged;
    pub use super::change_reviewed_0_1_2 as change_reviewed;
    pub use super::change_updated_0_1_2 as change_updated;
    pub use super::environment_created_0_1_1 as environment_created;
    pub use super::environment_deleted_0_1_1 as environment_deleted;
    pub use super::environment_modified_0_1_1 as environment_modified;
    pub use super::incident_detected_0_1_0 as incident_detected;
    pub use super::incident_reported_0_1_0 as incident_reported;
    pub use super::incident_resolved_0_1_0 as incident_resolved;
    pub use super::pipelinerun_finished_0_1_1 as pipelinerun_finished;
    pub use super::pipelinerun_queued_0_1_1 as pipelinerun_queued;
    pub use super::pipelinerun_started_0_1_1 as pipelinerun_started;
    pub use super::repository_created_0_1_1 as repository_created;
    pub use super::repository_deleted_0_1_1 as repository_deleted;
    pub use super::repository_modified_0_1_1 as repository_modified;
    pub use super::service_deployed_0_1_1 as service_deployed;
    pub use super::service_published_0_1_1 as service_published;
    pub use super::service_removed_0_1_1 as service_removed;
    pub use super::service_rolledback_0_1_1 as service_rolledback;
    pub use super::service_upgraded_0_1_1 as service_upgraded;
    pub use super::taskrun_finished_0_1_1 as taskrun_finished;
    pub use super::taskrun_started_0_1_1 as taskrun_started;
    pub use super::testcaserun_finished_0_1_0 as testcaserun_finished;
    pub use super::testcaserun_queued_0_1_0 as testcaserun_queued;
    pub use super::testcaserun_started_0_1_0 as testcaserun_started;
    pub use super::testoutput_published_0_1_0 as testoutput_published;
    pub use super::testsuiterun_finished_0_1_0 as testsuiterun_finished;
    pub use super::testsuiterun_queued_0_1_0 as testsuiterun_queued;
    pub use super::testsuiterun_started_0_1_0 as testsuiterun_started;
}
pub mod spec_0_4_0_draft {
    pub use super::artifact_signed_0_1_0 as artifact_signed;
    pub use super::branch_created_0_1_2 as branch_created;
    pub use super::branch_deleted_0_1_2 as branch_deleted;
    pub use super::build_finished_0_1_1 as build_finished;
    pub use super::build_queued_0_1_1 as build_queued;
    pub use super::build_started_0_1_1 as build_started;
    pub use super::change_abandoned_0_1_2 as change_abandoned;
    pub use super::change_created_0_2_0 as change_created;
    pub use super::change_merged_0_1_2 as change_merged;
    pub use super::change_reviewed_0_1_2 as change_reviewed;
    pub use super::change_updated_0_1_2 as change_updated;
    pub use super::environment_created_0_1_1 as environment_created;
    pub use super::environment_deleted_0_1_1 as environment_deleted;
    pub use super::environment_modified_0_1_1 as environment_modified;
    pub use super::incident_detected_0_1_0 as incident_detected;
    pub use super::incident_reported_0_1_0 as incident_reported;
    pub use super::incident_resolved_0_1_0 as incident_resolved;
    pub use super::pipelinerun_finished_0_1_1 as pipelinerun_finished;
    pub use super::pipelinerun_queued_0_1_1 as pipelinerun_queued;
    pub use super::pipelinerun_started_0_1_1 as pipelinerun_started;
    pub use super::repository_created_0_1_1 as repository_created;
    pub use super::repository_deleted_0_1_1 as repository_deleted;
    pub use super::repository_modified_0_1_1 as repository_modified;
    pub use super::service_deployed_0_1_1 as service_deployed;
    pub use super::service_published_0_1_1 as service_published;
    pub use super::service_removed_0_1_1 as service_removed;
    pub use super::service_rolledback_0_1_1 as service_rolledback;
    pub use super::service_upgraded_0_1_1 as service_upgraded;
    pub use super::taskrun_finished_0_1_1 as taskrun_finished;
    pub use super::taskrun_started_0_1_1 as taskrun_started;
    pub use super::testcaserun_finished_0_1_0 as testcaserun_finished;
    pub use super::testcaserun_queued_0_1_0 as testcaserun_queued;
    pub use super::testcaserun_started_0_1_0 as testcaserun_started;
    pub use super::testoutput_published_0_1_0 as testoutput_published;
    pub use super::testsuiterun_finished_0_1_0 as testsuiterun_finished;
    pub use super::testsuiterun_queued_0_1_0 as testsuiterun_queued;
    pub use super::testsuiterun_started_0_1_0 as testsuiterun_started;
}

pub const ARTIFACT_PACKAGED_0_1_1: &str = "dev.cdevents.artifact.packaged.0.1.1";
pub const ARTIFACT_PUBLISHED_0_1_1: &str = "dev.cdevents.artifact.published.0.1.1";
pub const ARTIFACT_SIGNED_0_1_0: &str = "dev.cdevents.artifact.signed.0.1.0";
pub const BRANCH_CREATED_0_1_2: &str = "dev.cdevents.branch.created.0.1.2";
pub const BRANCH_DELETED_0_1_2: &str = "dev.cdevents.branch.deleted.0.1.2";
pub const BUILD_FINISHED_0_1_1: &str = "dev.cdevents.build.finished.0.1.1";
pub const BUILD_QUEUED_0_1_1: &str = "dev.cdevents.build.queued.0.1.1";
pub const BUILD_STARTED_0_1_1: &str = "dev.cdevents.build.started.0.1.1";
pub const CHANGE_ABANDONED_0_1_2: &str = "dev.cdevents.change.abandoned.0.1.2";
pub const CHANGE_CREATED_0_2_0: &str = "dev.cdevents.change.created.0.2.0";
pub const CHANGE_CREATED_0_1_2: &str = "dev.cdevents.change.created.0.1.2";
pub const CHANGE_MERGED_0_1_2: &str = "dev.cdevents.change.merged.0.1.2";
pub const CHANGE_REVIEWED_0_1_2: &str = "dev.cdevents.change.reviewed.0.1.2";
pub const CHANGE_UPDATED_0_1_2: &str = "dev.cdevents.change.updated.0.1.2";
pub const ENVIRONMENT_CREATED_0_1_1: &str = "dev.cdevents.environment.created.0.1.1";
pub const ENVIRONMENT_DELETED_0_1_1: &str = "dev.cdevents.environment.deleted.0.1.1";
pub const ENVIRONMENT_MODIFIED_0_1_1: &str = "dev.cdevents.environment.modified.0.1.1";
pub const INCIDENT_DETECTED_0_1_0: &str = "dev.cdevents.incident.detected.0.1.0";
pub const INCIDENT_REPORTED_0_1_0: &str = "dev.cdevents.incident.reported.0.1.0";
pub const INCIDENT_RESOLVED_0_1_0: &str = "dev.cdevents.incident.resolved.0.1.0";
pub const PIPELINERUN_FINISHED_0_1_1: &str = "dev.cdevents.pipelinerun.finished.0.1.1";
pub const PIPELINERUN_QUEUED_0_1_1: &str = "dev.cdevents.pipelinerun.queued.0.1.1";
pub const PIPELINERUN_STARTED_0_1_1: &str = "dev.cdevents.pipelinerun.started.0.1.1";
pub const REPOSITORY_CREATED_0_1_1: &str = "dev.cdevents.repository.created.0.1.1";
pub const REPOSITORY_DELETED_0_1_1: &str = "dev.cdevents.repository.deleted.0.1.1";
pub const REPOSITORY_MODIFIED_0_1_1: &str = "dev.cdevents.repository.modified.0.1.1";
pub const SERVICE_DEPLOYED_0_1_1: &str = "dev.cdevents.service.deployed.0.1.1";
pub const SERVICE_PUBLISHED_0_1_1: &str = "dev.cdevents.service.published.0.1.1";
pub const SERVICE_REMOVED_0_1_1: &str = "dev.cdevents.service.removed.0.1.1";
pub const SERVICE_ROLLEDBACK_0_1_1: &str = "dev.cdevents.service.rolledback.0.1.1";
pub const SERVICE_UPGRADED_0_1_1: &str = "dev.cdevents.service.upgraded.0.1.1";
pub const TASKRUN_FINISHED_0_1_1: &str = "dev.cdevents.taskrun.finished.0.1.1";
pub const TASKRUN_STARTED_0_1_1: &str = "dev.cdevents.taskrun.started.0.1.1";
pub const TESTCASERUN_FINISHED_0_1_0: &str = "dev.cdevents.testcaserun.finished.0.1.0";
pub const TESTCASERUN_QUEUED_0_1_0: &str = "dev.cdevents.testcaserun.queued.0.1.0";
pub const TESTCASERUN_STARTED_0_1_0: &str = "dev.cdevents.testcaserun.started.0.1.0";
pub const TESTOUTPUT_PUBLISHED_0_1_0: &str = "dev.cdevents.testoutput.published.0.1.0";
pub const TESTSUITERUN_FINISHED_0_1_0: &str = "dev.cdevents.testsuiterun.finished.0.1.0";
pub const TESTSUITERUN_QUEUED_0_1_0: &str = "dev.cdevents.testsuiterun.queued.0.1.0";
pub const TESTSUITERUN_STARTED_0_1_0: &str = "dev.cdevents.testsuiterun.started.0.1.0";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)] // TODO how to use content of context.type as discriminator ?
pub enum Content {
    ArtifactPackaged011(artifact_packaged_0_1_1::Content),
    ArtifactPublished011(artifact_published_0_1_1::Content),
    ArtifactSigned010(artifact_signed_0_1_0::Content),
    BranchCreated012(branch_created_0_1_2::Content),
    BranchDeleted012(branch_deleted_0_1_2::Content),
    BuildFinished011(build_finished_0_1_1::Content),
    BuildQueued011(build_queued_0_1_1::Content),
    BuildStarted011(build_started_0_1_1::Content),
    ChangeAbandoned012(change_abandoned_0_1_2::Content),
    ChangeCreated020(change_created_0_2_0::Content),
    ChangeCreated012(change_created_0_1_2::Content),
    ChangeMerged012(change_merged_0_1_2::Content),
    ChangeReviewed012(change_reviewed_0_1_2::Content),
    ChangeUpdated012(change_updated_0_1_2::Content),
    EnvironmentCreated011(environment_created_0_1_1::Content),
    EnvironmentDeleted011(environment_deleted_0_1_1::Content),
    EnvironmentModified011(environment_modified_0_1_1::Content),
    IncidentDetected010(incident_detected_0_1_0::Content),
    IncidentReported010(incident_reported_0_1_0::Content),
    IncidentResolved010(incident_resolved_0_1_0::Content),
    PipelinerunFinished011(pipelinerun_finished_0_1_1::Content),
    PipelinerunQueued011(pipelinerun_queued_0_1_1::Content),
    PipelinerunStarted011(pipelinerun_started_0_1_1::Content),
    RepositoryCreated011(repository_created_0_1_1::Content),
    RepositoryDeleted011(repository_deleted_0_1_1::Content),
    RepositoryModified011(repository_modified_0_1_1::Content),
    ServiceDeployed011(service_deployed_0_1_1::Content),
    ServicePublished011(service_published_0_1_1::Content),
    ServiceRemoved011(service_removed_0_1_1::Content),
    ServiceRolledback011(service_rolledback_0_1_1::Content),
    ServiceUpgraded011(service_upgraded_0_1_1::Content),
    TaskrunFinished011(taskrun_finished_0_1_1::Content),
    TaskrunStarted011(taskrun_started_0_1_1::Content),
    TestcaserunFinished010(testcaserun_finished_0_1_0::Content),
    TestcaserunQueued010(testcaserun_queued_0_1_0::Content),
    TestcaserunStarted010(testcaserun_started_0_1_0::Content),
    TestoutputPublished010(testoutput_published_0_1_0::Content),
    TestsuiterunFinished010(testsuiterun_finished_0_1_0::Content),
    TestsuiterunQueued010(testsuiterun_queued_0_1_0::Content),
    TestsuiterunStarted010(testsuiterun_started_0_1_0::Content),
}

impl Content {
    pub fn from_json(ty: &str, json: serde_json::Value) -> Result<Self, serde_json::Error>{
        match ty {
            ARTIFACT_PACKAGED_0_1_1 => {
                let variant: artifact_packaged_0_1_1::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            ARTIFACT_PUBLISHED_0_1_1 => {
                let variant: artifact_published_0_1_1::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            ARTIFACT_SIGNED_0_1_0 => {
                let variant: artifact_signed_0_1_0::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            BRANCH_CREATED_0_1_2 => {
                let variant: branch_created_0_1_2::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            BRANCH_DELETED_0_1_2 => {
                let variant: branch_deleted_0_1_2::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            BUILD_FINISHED_0_1_1 => {
                let variant: build_finished_0_1_1::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            BUILD_QUEUED_0_1_1 => {
                let variant: build_queued_0_1_1::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            BUILD_STARTED_0_1_1 => {
                let variant: build_started_0_1_1::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            CHANGE_ABANDONED_0_1_2 => {
                let variant: change_abandoned_0_1_2::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            CHANGE_CREATED_0_2_0 => {
                let variant: change_created_0_2_0::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            CHANGE_CREATED_0_1_2 => {
                let variant: change_created_0_1_2::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            CHANGE_MERGED_0_1_2 => {
                let variant: change_merged_0_1_2::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            CHANGE_REVIEWED_0_1_2 => {
                let variant: change_reviewed_0_1_2::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            CHANGE_UPDATED_0_1_2 => {
                let variant: change_updated_0_1_2::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            ENVIRONMENT_CREATED_0_1_1 => {
                let variant: environment_created_0_1_1::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            ENVIRONMENT_DELETED_0_1_1 => {
                let variant: environment_deleted_0_1_1::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            ENVIRONMENT_MODIFIED_0_1_1 => {
                let variant: environment_modified_0_1_1::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            INCIDENT_DETECTED_0_1_0 => {
                let variant: incident_detected_0_1_0::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            INCIDENT_REPORTED_0_1_0 => {
                let variant: incident_reported_0_1_0::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            INCIDENT_RESOLVED_0_1_0 => {
                let variant: incident_resolved_0_1_0::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            PIPELINERUN_FINISHED_0_1_1 => {
                let variant: pipelinerun_finished_0_1_1::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            PIPELINERUN_QUEUED_0_1_1 => {
                let variant: pipelinerun_queued_0_1_1::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            PIPELINERUN_STARTED_0_1_1 => {
                let variant: pipelinerun_started_0_1_1::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            REPOSITORY_CREATED_0_1_1 => {
                let variant: repository_created_0_1_1::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            REPOSITORY_DELETED_0_1_1 => {
                let variant: repository_deleted_0_1_1::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            REPOSITORY_MODIFIED_0_1_1 => {
                let variant: repository_modified_0_1_1::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            SERVICE_DEPLOYED_0_1_1 => {
                let variant: service_deployed_0_1_1::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            SERVICE_PUBLISHED_0_1_1 => {
                let variant: service_published_0_1_1::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            SERVICE_REMOVED_0_1_1 => {
                let variant: service_removed_0_1_1::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            SERVICE_ROLLEDBACK_0_1_1 => {
                let variant: service_rolledback_0_1_1::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            SERVICE_UPGRADED_0_1_1 => {
                let variant: service_upgraded_0_1_1::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            TASKRUN_FINISHED_0_1_1 => {
                let variant: taskrun_finished_0_1_1::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            TASKRUN_STARTED_0_1_1 => {
                let variant: taskrun_started_0_1_1::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            TESTCASERUN_FINISHED_0_1_0 => {
                let variant: testcaserun_finished_0_1_0::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            TESTCASERUN_QUEUED_0_1_0 => {
                let variant: testcaserun_queued_0_1_0::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            TESTCASERUN_STARTED_0_1_0 => {
                let variant: testcaserun_started_0_1_0::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            TESTOUTPUT_PUBLISHED_0_1_0 => {
                let variant: testoutput_published_0_1_0::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            TESTSUITERUN_FINISHED_0_1_0 => {
                let variant: testsuiterun_finished_0_1_0::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            TESTSUITERUN_QUEUED_0_1_0 => {
                let variant: testsuiterun_queued_0_1_0::Content = serde_json::from_value(json)?;
                Ok(variant.into())
            },
            TESTSUITERUN_STARTED_0_1_0 => {
                let variant: testsuiterun_started_0_1_0::Content = serde_json::from_value(json)?;
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
            Self::ArtifactPackaged011(_) => ARTIFACT_PACKAGED_0_1_1,
            Self::ArtifactPublished011(_) => ARTIFACT_PUBLISHED_0_1_1,
            Self::ArtifactSigned010(_) => ARTIFACT_SIGNED_0_1_0,
            Self::BranchCreated012(_) => BRANCH_CREATED_0_1_2,
            Self::BranchDeleted012(_) => BRANCH_DELETED_0_1_2,
            Self::BuildFinished011(_) => BUILD_FINISHED_0_1_1,
            Self::BuildQueued011(_) => BUILD_QUEUED_0_1_1,
            Self::BuildStarted011(_) => BUILD_STARTED_0_1_1,
            Self::ChangeAbandoned012(_) => CHANGE_ABANDONED_0_1_2,
            Self::ChangeCreated020(_) => CHANGE_CREATED_0_2_0,
            Self::ChangeCreated012(_) => CHANGE_CREATED_0_1_2,
            Self::ChangeMerged012(_) => CHANGE_MERGED_0_1_2,
            Self::ChangeReviewed012(_) => CHANGE_REVIEWED_0_1_2,
            Self::ChangeUpdated012(_) => CHANGE_UPDATED_0_1_2,
            Self::EnvironmentCreated011(_) => ENVIRONMENT_CREATED_0_1_1,
            Self::EnvironmentDeleted011(_) => ENVIRONMENT_DELETED_0_1_1,
            Self::EnvironmentModified011(_) => ENVIRONMENT_MODIFIED_0_1_1,
            Self::IncidentDetected010(_) => INCIDENT_DETECTED_0_1_0,
            Self::IncidentReported010(_) => INCIDENT_REPORTED_0_1_0,
            Self::IncidentResolved010(_) => INCIDENT_RESOLVED_0_1_0,
            Self::PipelinerunFinished011(_) => PIPELINERUN_FINISHED_0_1_1,
            Self::PipelinerunQueued011(_) => PIPELINERUN_QUEUED_0_1_1,
            Self::PipelinerunStarted011(_) => PIPELINERUN_STARTED_0_1_1,
            Self::RepositoryCreated011(_) => REPOSITORY_CREATED_0_1_1,
            Self::RepositoryDeleted011(_) => REPOSITORY_DELETED_0_1_1,
            Self::RepositoryModified011(_) => REPOSITORY_MODIFIED_0_1_1,
            Self::ServiceDeployed011(_) => SERVICE_DEPLOYED_0_1_1,
            Self::ServicePublished011(_) => SERVICE_PUBLISHED_0_1_1,
            Self::ServiceRemoved011(_) => SERVICE_REMOVED_0_1_1,
            Self::ServiceRolledback011(_) => SERVICE_ROLLEDBACK_0_1_1,
            Self::ServiceUpgraded011(_) => SERVICE_UPGRADED_0_1_1,
            Self::TaskrunFinished011(_) => TASKRUN_FINISHED_0_1_1,
            Self::TaskrunStarted011(_) => TASKRUN_STARTED_0_1_1,
            Self::TestcaserunFinished010(_) => TESTCASERUN_FINISHED_0_1_0,
            Self::TestcaserunQueued010(_) => TESTCASERUN_QUEUED_0_1_0,
            Self::TestcaserunStarted010(_) => TESTCASERUN_STARTED_0_1_0,
            Self::TestoutputPublished010(_) => TESTOUTPUT_PUBLISHED_0_1_0,
            Self::TestsuiterunFinished010(_) => TESTSUITERUN_FINISHED_0_1_0,
            Self::TestsuiterunQueued010(_) => TESTSUITERUN_QUEUED_0_1_0,
            Self::TestsuiterunStarted010(_) => TESTSUITERUN_STARTED_0_1_0,
        }
    }

    pub fn subject(&self) -> &'static str {
        match self {
            Self::ArtifactPackaged011(_) => "artifact",
            Self::ArtifactPublished011(_) => "artifact",
            Self::ArtifactSigned010(_) => "artifact",
            Self::BranchCreated012(_) => "branch",
            Self::BranchDeleted012(_) => "branch",
            Self::BuildFinished011(_) => "build",
            Self::BuildQueued011(_) => "build",
            Self::BuildStarted011(_) => "build",
            Self::ChangeAbandoned012(_) => "change",
            Self::ChangeCreated020(_) => "change",
            Self::ChangeCreated012(_) => "change",
            Self::ChangeMerged012(_) => "change",
            Self::ChangeReviewed012(_) => "change",
            Self::ChangeUpdated012(_) => "change",
            Self::EnvironmentCreated011(_) => "environment",
            Self::EnvironmentDeleted011(_) => "environment",
            Self::EnvironmentModified011(_) => "environment",
            Self::IncidentDetected010(_) => "incident",
            Self::IncidentReported010(_) => "incident",
            Self::IncidentResolved010(_) => "incident",
            Self::PipelinerunFinished011(_) => "pipelineRun",
            Self::PipelinerunQueued011(_) => "pipelineRun",
            Self::PipelinerunStarted011(_) => "pipelineRun",
            Self::RepositoryCreated011(_) => "repository",
            Self::RepositoryDeleted011(_) => "repository",
            Self::RepositoryModified011(_) => "repository",
            Self::ServiceDeployed011(_) => "service",
            Self::ServicePublished011(_) => "service",
            Self::ServiceRemoved011(_) => "service",
            Self::ServiceRolledback011(_) => "service",
            Self::ServiceUpgraded011(_) => "service",
            Self::TaskrunFinished011(_) => "taskRun",
            Self::TaskrunStarted011(_) => "taskRun",
            Self::TestcaserunFinished010(_) => "testCaseRun",
            Self::TestcaserunQueued010(_) => "testCaseRun",
            Self::TestcaserunStarted010(_) => "testCaseRun",
            Self::TestoutputPublished010(_) => "testOutput",
            Self::TestsuiterunFinished010(_) => "testSuiteRun",
            Self::TestsuiterunQueued010(_) => "testSuiteRun",
            Self::TestsuiterunStarted010(_) => "testSuiteRun",
        }
    }

    pub fn predicate(&self) -> &'static str {
        match self {
            Self::ArtifactPackaged011(_) => "packaged",
            Self::ArtifactPublished011(_) => "published",
            Self::ArtifactSigned010(_) => "signed",
            Self::BranchCreated012(_) => "created",
            Self::BranchDeleted012(_) => "deleted",
            Self::BuildFinished011(_) => "finished",
            Self::BuildQueued011(_) => "queued",
            Self::BuildStarted011(_) => "started",
            Self::ChangeAbandoned012(_) => "abandoned",
            Self::ChangeCreated020(_) => "created",
            Self::ChangeCreated012(_) => "created",
            Self::ChangeMerged012(_) => "merged",
            Self::ChangeReviewed012(_) => "reviewed",
            Self::ChangeUpdated012(_) => "updated",
            Self::EnvironmentCreated011(_) => "created",
            Self::EnvironmentDeleted011(_) => "deleted",
            Self::EnvironmentModified011(_) => "modified",
            Self::IncidentDetected010(_) => "detected",
            Self::IncidentReported010(_) => "reported",
            Self::IncidentResolved010(_) => "resolved",
            Self::PipelinerunFinished011(_) => "finished",
            Self::PipelinerunQueued011(_) => "queued",
            Self::PipelinerunStarted011(_) => "started",
            Self::RepositoryCreated011(_) => "created",
            Self::RepositoryDeleted011(_) => "deleted",
            Self::RepositoryModified011(_) => "modified",
            Self::ServiceDeployed011(_) => "deployed",
            Self::ServicePublished011(_) => "published",
            Self::ServiceRemoved011(_) => "removed",
            Self::ServiceRolledback011(_) => "rolledback",
            Self::ServiceUpgraded011(_) => "upgraded",
            Self::TaskrunFinished011(_) => "finished",
            Self::TaskrunStarted011(_) => "started",
            Self::TestcaserunFinished010(_) => "finished",
            Self::TestcaserunQueued010(_) => "queued",
            Self::TestcaserunStarted010(_) => "started",
            Self::TestoutputPublished010(_) => "published",
            Self::TestsuiterunFinished010(_) => "finished",
            Self::TestsuiterunQueued010(_) => "queued",
            Self::TestsuiterunStarted010(_) => "started",
        }
    }
}

// due to inconstency in case/format the subject could be not be extracted from the context.type (ty), jsonshema $id, spec filename (shema, examples)
pub fn extract_subject_predicate(ty: &str) -> Option<(&str, &str)>{
    // let mut split = ty.split('.');
    match ty {
        ARTIFACT_PACKAGED_0_1_1 => Some(("artifact", "packaged")),
        ARTIFACT_PUBLISHED_0_1_1 => Some(("artifact", "published")),
        ARTIFACT_SIGNED_0_1_0 => Some(("artifact", "signed")),
        BRANCH_CREATED_0_1_2 => Some(("branch", "created")),
        BRANCH_DELETED_0_1_2 => Some(("branch", "deleted")),
        BUILD_FINISHED_0_1_1 => Some(("build", "finished")),
        BUILD_QUEUED_0_1_1 => Some(("build", "queued")),
        BUILD_STARTED_0_1_1 => Some(("build", "started")),
        CHANGE_ABANDONED_0_1_2 => Some(("change", "abandoned")),
        CHANGE_CREATED_0_2_0 => Some(("change", "created")),
        CHANGE_CREATED_0_1_2 => Some(("change", "created")),
        CHANGE_MERGED_0_1_2 => Some(("change", "merged")),
        CHANGE_REVIEWED_0_1_2 => Some(("change", "reviewed")),
        CHANGE_UPDATED_0_1_2 => Some(("change", "updated")),
        ENVIRONMENT_CREATED_0_1_1 => Some(("environment", "created")),
        ENVIRONMENT_DELETED_0_1_1 => Some(("environment", "deleted")),
        ENVIRONMENT_MODIFIED_0_1_1 => Some(("environment", "modified")),
        INCIDENT_DETECTED_0_1_0 => Some(("incident", "detected")),
        INCIDENT_REPORTED_0_1_0 => Some(("incident", "reported")),
        INCIDENT_RESOLVED_0_1_0 => Some(("incident", "resolved")),
        PIPELINERUN_FINISHED_0_1_1 => Some(("pipelineRun", "finished")),
        PIPELINERUN_QUEUED_0_1_1 => Some(("pipelineRun", "queued")),
        PIPELINERUN_STARTED_0_1_1 => Some(("pipelineRun", "started")),
        REPOSITORY_CREATED_0_1_1 => Some(("repository", "created")),
        REPOSITORY_DELETED_0_1_1 => Some(("repository", "deleted")),
        REPOSITORY_MODIFIED_0_1_1 => Some(("repository", "modified")),
        SERVICE_DEPLOYED_0_1_1 => Some(("service", "deployed")),
        SERVICE_PUBLISHED_0_1_1 => Some(("service", "published")),
        SERVICE_REMOVED_0_1_1 => Some(("service", "removed")),
        SERVICE_ROLLEDBACK_0_1_1 => Some(("service", "rolledback")),
        SERVICE_UPGRADED_0_1_1 => Some(("service", "upgraded")),
        TASKRUN_FINISHED_0_1_1 => Some(("taskRun", "finished")),
        TASKRUN_STARTED_0_1_1 => Some(("taskRun", "started")),
        TESTCASERUN_FINISHED_0_1_0 => Some(("testCaseRun", "finished")),
        TESTCASERUN_QUEUED_0_1_0 => Some(("testCaseRun", "queued")),
        TESTCASERUN_STARTED_0_1_0 => Some(("testCaseRun", "started")),
        TESTOUTPUT_PUBLISHED_0_1_0 => Some(("testOutput", "published")),
        TESTSUITERUN_FINISHED_0_1_0 => Some(("testSuiteRun", "finished")),
        TESTSUITERUN_QUEUED_0_1_0 => Some(("testSuiteRun", "queued")),
        TESTSUITERUN_STARTED_0_1_0 => Some(("testSuiteRun", "started")),
        _ => None,
    }
}

impl From<artifact_packaged_0_1_1::Content> for Content {
    fn from(value: artifact_packaged_0_1_1::Content) -> Self {
        Self::ArtifactPackaged011(value)
    }
}
impl From<artifact_published_0_1_1::Content> for Content {
    fn from(value: artifact_published_0_1_1::Content) -> Self {
        Self::ArtifactPublished011(value)
    }
}
impl From<artifact_signed_0_1_0::Content> for Content {
    fn from(value: artifact_signed_0_1_0::Content) -> Self {
        Self::ArtifactSigned010(value)
    }
}
impl From<branch_created_0_1_2::Content> for Content {
    fn from(value: branch_created_0_1_2::Content) -> Self {
        Self::BranchCreated012(value)
    }
}
impl From<branch_deleted_0_1_2::Content> for Content {
    fn from(value: branch_deleted_0_1_2::Content) -> Self {
        Self::BranchDeleted012(value)
    }
}
impl From<build_finished_0_1_1::Content> for Content {
    fn from(value: build_finished_0_1_1::Content) -> Self {
        Self::BuildFinished011(value)
    }
}
impl From<build_queued_0_1_1::Content> for Content {
    fn from(value: build_queued_0_1_1::Content) -> Self {
        Self::BuildQueued011(value)
    }
}
impl From<build_started_0_1_1::Content> for Content {
    fn from(value: build_started_0_1_1::Content) -> Self {
        Self::BuildStarted011(value)
    }
}
impl From<change_abandoned_0_1_2::Content> for Content {
    fn from(value: change_abandoned_0_1_2::Content) -> Self {
        Self::ChangeAbandoned012(value)
    }
}
impl From<change_created_0_2_0::Content> for Content {
    fn from(value: change_created_0_2_0::Content) -> Self {
        Self::ChangeCreated020(value)
    }
}
impl From<change_created_0_1_2::Content> for Content {
    fn from(value: change_created_0_1_2::Content) -> Self {
        Self::ChangeCreated012(value)
    }
}
impl From<change_merged_0_1_2::Content> for Content {
    fn from(value: change_merged_0_1_2::Content) -> Self {
        Self::ChangeMerged012(value)
    }
}
impl From<change_reviewed_0_1_2::Content> for Content {
    fn from(value: change_reviewed_0_1_2::Content) -> Self {
        Self::ChangeReviewed012(value)
    }
}
impl From<change_updated_0_1_2::Content> for Content {
    fn from(value: change_updated_0_1_2::Content) -> Self {
        Self::ChangeUpdated012(value)
    }
}
impl From<environment_created_0_1_1::Content> for Content {
    fn from(value: environment_created_0_1_1::Content) -> Self {
        Self::EnvironmentCreated011(value)
    }
}
impl From<environment_deleted_0_1_1::Content> for Content {
    fn from(value: environment_deleted_0_1_1::Content) -> Self {
        Self::EnvironmentDeleted011(value)
    }
}
impl From<environment_modified_0_1_1::Content> for Content {
    fn from(value: environment_modified_0_1_1::Content) -> Self {
        Self::EnvironmentModified011(value)
    }
}
impl From<incident_detected_0_1_0::Content> for Content {
    fn from(value: incident_detected_0_1_0::Content) -> Self {
        Self::IncidentDetected010(value)
    }
}
impl From<incident_reported_0_1_0::Content> for Content {
    fn from(value: incident_reported_0_1_0::Content) -> Self {
        Self::IncidentReported010(value)
    }
}
impl From<incident_resolved_0_1_0::Content> for Content {
    fn from(value: incident_resolved_0_1_0::Content) -> Self {
        Self::IncidentResolved010(value)
    }
}
impl From<pipelinerun_finished_0_1_1::Content> for Content {
    fn from(value: pipelinerun_finished_0_1_1::Content) -> Self {
        Self::PipelinerunFinished011(value)
    }
}
impl From<pipelinerun_queued_0_1_1::Content> for Content {
    fn from(value: pipelinerun_queued_0_1_1::Content) -> Self {
        Self::PipelinerunQueued011(value)
    }
}
impl From<pipelinerun_started_0_1_1::Content> for Content {
    fn from(value: pipelinerun_started_0_1_1::Content) -> Self {
        Self::PipelinerunStarted011(value)
    }
}
impl From<repository_created_0_1_1::Content> for Content {
    fn from(value: repository_created_0_1_1::Content) -> Self {
        Self::RepositoryCreated011(value)
    }
}
impl From<repository_deleted_0_1_1::Content> for Content {
    fn from(value: repository_deleted_0_1_1::Content) -> Self {
        Self::RepositoryDeleted011(value)
    }
}
impl From<repository_modified_0_1_1::Content> for Content {
    fn from(value: repository_modified_0_1_1::Content) -> Self {
        Self::RepositoryModified011(value)
    }
}
impl From<service_deployed_0_1_1::Content> for Content {
    fn from(value: service_deployed_0_1_1::Content) -> Self {
        Self::ServiceDeployed011(value)
    }
}
impl From<service_published_0_1_1::Content> for Content {
    fn from(value: service_published_0_1_1::Content) -> Self {
        Self::ServicePublished011(value)
    }
}
impl From<service_removed_0_1_1::Content> for Content {
    fn from(value: service_removed_0_1_1::Content) -> Self {
        Self::ServiceRemoved011(value)
    }
}
impl From<service_rolledback_0_1_1::Content> for Content {
    fn from(value: service_rolledback_0_1_1::Content) -> Self {
        Self::ServiceRolledback011(value)
    }
}
impl From<service_upgraded_0_1_1::Content> for Content {
    fn from(value: service_upgraded_0_1_1::Content) -> Self {
        Self::ServiceUpgraded011(value)
    }
}
impl From<taskrun_finished_0_1_1::Content> for Content {
    fn from(value: taskrun_finished_0_1_1::Content) -> Self {
        Self::TaskrunFinished011(value)
    }
}
impl From<taskrun_started_0_1_1::Content> for Content {
    fn from(value: taskrun_started_0_1_1::Content) -> Self {
        Self::TaskrunStarted011(value)
    }
}
impl From<testcaserun_finished_0_1_0::Content> for Content {
    fn from(value: testcaserun_finished_0_1_0::Content) -> Self {
        Self::TestcaserunFinished010(value)
    }
}
impl From<testcaserun_queued_0_1_0::Content> for Content {
    fn from(value: testcaserun_queued_0_1_0::Content) -> Self {
        Self::TestcaserunQueued010(value)
    }
}
impl From<testcaserun_started_0_1_0::Content> for Content {
    fn from(value: testcaserun_started_0_1_0::Content) -> Self {
        Self::TestcaserunStarted010(value)
    }
}
impl From<testoutput_published_0_1_0::Content> for Content {
    fn from(value: testoutput_published_0_1_0::Content) -> Self {
        Self::TestoutputPublished010(value)
    }
}
impl From<testsuiterun_finished_0_1_0::Content> for Content {
    fn from(value: testsuiterun_finished_0_1_0::Content) -> Self {
        Self::TestsuiterunFinished010(value)
    }
}
impl From<testsuiterun_queued_0_1_0::Content> for Content {
    fn from(value: testsuiterun_queued_0_1_0::Content) -> Self {
        Self::TestsuiterunQueued010(value)
    }
}
impl From<testsuiterun_started_0_1_0::Content> for Content {
    fn from(value: testsuiterun_started_0_1_0::Content) -> Self {
        Self::TestsuiterunStarted010(value)
    }
}

#[cfg(feature = "testkit")]
impl<> proptest::arbitrary::Arbitrary for Content {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        prop_oneof![
            any::<artifact_packaged_0_1_1::Content>().prop_map(Content::from),
            any::<artifact_published_0_1_1::Content>().prop_map(Content::from),
            any::<artifact_signed_0_1_0::Content>().prop_map(Content::from),
            any::<branch_created_0_1_2::Content>().prop_map(Content::from),
            any::<branch_deleted_0_1_2::Content>().prop_map(Content::from),
            any::<build_finished_0_1_1::Content>().prop_map(Content::from),
            any::<build_queued_0_1_1::Content>().prop_map(Content::from),
            any::<build_started_0_1_1::Content>().prop_map(Content::from),
            any::<change_abandoned_0_1_2::Content>().prop_map(Content::from),
            any::<change_created_0_2_0::Content>().prop_map(Content::from),
            any::<change_created_0_1_2::Content>().prop_map(Content::from),
            any::<change_merged_0_1_2::Content>().prop_map(Content::from),
            any::<change_reviewed_0_1_2::Content>().prop_map(Content::from),
            any::<change_updated_0_1_2::Content>().prop_map(Content::from),
            any::<environment_created_0_1_1::Content>().prop_map(Content::from),
            any::<environment_deleted_0_1_1::Content>().prop_map(Content::from),
            any::<environment_modified_0_1_1::Content>().prop_map(Content::from),
            any::<incident_detected_0_1_0::Content>().prop_map(Content::from),
            any::<incident_reported_0_1_0::Content>().prop_map(Content::from),
            any::<incident_resolved_0_1_0::Content>().prop_map(Content::from),
            any::<pipelinerun_finished_0_1_1::Content>().prop_map(Content::from),
            any::<pipelinerun_queued_0_1_1::Content>().prop_map(Content::from),
            any::<pipelinerun_started_0_1_1::Content>().prop_map(Content::from),
            any::<repository_created_0_1_1::Content>().prop_map(Content::from),
            any::<repository_deleted_0_1_1::Content>().prop_map(Content::from),
            any::<repository_modified_0_1_1::Content>().prop_map(Content::from),
            any::<service_deployed_0_1_1::Content>().prop_map(Content::from),
            any::<service_published_0_1_1::Content>().prop_map(Content::from),
            any::<service_removed_0_1_1::Content>().prop_map(Content::from),
            any::<service_rolledback_0_1_1::Content>().prop_map(Content::from),
            any::<service_upgraded_0_1_1::Content>().prop_map(Content::from),
            any::<taskrun_finished_0_1_1::Content>().prop_map(Content::from),
            any::<taskrun_started_0_1_1::Content>().prop_map(Content::from),
            any::<testcaserun_finished_0_1_0::Content>().prop_map(Content::from),
            any::<testcaserun_queued_0_1_0::Content>().prop_map(Content::from),
            any::<testcaserun_started_0_1_0::Content>().prop_map(Content::from),
            any::<testoutput_published_0_1_0::Content>().prop_map(Content::from),
            any::<testsuiterun_finished_0_1_0::Content>().prop_map(Content::from),
            any::<testsuiterun_queued_0_1_0::Content>().prop_map(Content::from),
            any::<testsuiterun_started_0_1_0::Content>().prop_map(Content::from),
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
//         assert_eq!(extract_subject_predicate(ARTIFACT_PACKAGED_0_1_1), Some(("artifact","packaged")));
//         
//         assert_eq!(extract_subject_predicate(ARTIFACT_PUBLISHED_0_1_1), Some(("artifact","published")));
//         
//         assert_eq!(extract_subject_predicate(ARTIFACT_SIGNED_0_1_0), Some(("artifact","signed")));
//         
//         assert_eq!(extract_subject_predicate(BRANCH_CREATED_0_1_2), Some(("branch","created")));
//         
//         assert_eq!(extract_subject_predicate(BRANCH_DELETED_0_1_2), Some(("branch","deleted")));
//         
//         assert_eq!(extract_subject_predicate(BUILD_FINISHED_0_1_1), Some(("build","finished")));
//         
//         assert_eq!(extract_subject_predicate(BUILD_QUEUED_0_1_1), Some(("build","queued")));
//         
//         assert_eq!(extract_subject_predicate(BUILD_STARTED_0_1_1), Some(("build","started")));
//         
//         assert_eq!(extract_subject_predicate(CHANGE_ABANDONED_0_1_2), Some(("change","abandoned")));
//         
//         assert_eq!(extract_subject_predicate(CHANGE_CREATED_0_2_0), Some(("change","created")));
//         
//         assert_eq!(extract_subject_predicate(CHANGE_CREATED_0_1_2), Some(("change","created")));
//         
//         assert_eq!(extract_subject_predicate(CHANGE_MERGED_0_1_2), Some(("change","merged")));
//         
//         assert_eq!(extract_subject_predicate(CHANGE_REVIEWED_0_1_2), Some(("change","reviewed")));
//         
//         assert_eq!(extract_subject_predicate(CHANGE_UPDATED_0_1_2), Some(("change","updated")));
//         
//         assert_eq!(extract_subject_predicate(ENVIRONMENT_CREATED_0_1_1), Some(("environment","created")));
//         
//         assert_eq!(extract_subject_predicate(ENVIRONMENT_DELETED_0_1_1), Some(("environment","deleted")));
//         
//         assert_eq!(extract_subject_predicate(ENVIRONMENT_MODIFIED_0_1_1), Some(("environment","modified")));
//         
//         assert_eq!(extract_subject_predicate(INCIDENT_DETECTED_0_1_0), Some(("incident","detected")));
//         
//         assert_eq!(extract_subject_predicate(INCIDENT_REPORTED_0_1_0), Some(("incident","reported")));
//         
//         assert_eq!(extract_subject_predicate(INCIDENT_RESOLVED_0_1_0), Some(("incident","resolved")));
//         
//         assert_eq!(extract_subject_predicate(PIPELINERUN_FINISHED_0_1_1), Some(("pipelinerun","finished")));
//         
//         assert_eq!(extract_subject_predicate(PIPELINERUN_QUEUED_0_1_1), Some(("pipelinerun","queued")));
//         
//         assert_eq!(extract_subject_predicate(PIPELINERUN_STARTED_0_1_1), Some(("pipelinerun","started")));
//         
//         assert_eq!(extract_subject_predicate(REPOSITORY_CREATED_0_1_1), Some(("repository","created")));
//         
//         assert_eq!(extract_subject_predicate(REPOSITORY_DELETED_0_1_1), Some(("repository","deleted")));
//         
//         assert_eq!(extract_subject_predicate(REPOSITORY_MODIFIED_0_1_1), Some(("repository","modified")));
//         
//         assert_eq!(extract_subject_predicate(SERVICE_DEPLOYED_0_1_1), Some(("service","deployed")));
//         
//         assert_eq!(extract_subject_predicate(SERVICE_PUBLISHED_0_1_1), Some(("service","published")));
//         
//         assert_eq!(extract_subject_predicate(SERVICE_REMOVED_0_1_1), Some(("service","removed")));
//         
//         assert_eq!(extract_subject_predicate(SERVICE_ROLLEDBACK_0_1_1), Some(("service","rolledback")));
//         
//         assert_eq!(extract_subject_predicate(SERVICE_UPGRADED_0_1_1), Some(("service","upgraded")));
//         
//         assert_eq!(extract_subject_predicate(TASKRUN_FINISHED_0_1_1), Some(("taskrun","finished")));
//         
//         assert_eq!(extract_subject_predicate(TASKRUN_STARTED_0_1_1), Some(("taskrun","started")));
//         
//         assert_eq!(extract_subject_predicate(TESTCASERUN_FINISHED_0_1_0), Some(("testcaserun","finished")));
//         
//         assert_eq!(extract_subject_predicate(TESTCASERUN_QUEUED_0_1_0), Some(("testcaserun","queued")));
//         
//         assert_eq!(extract_subject_predicate(TESTCASERUN_STARTED_0_1_0), Some(("testcaserun","started")));
//         
//         assert_eq!(extract_subject_predicate(TESTOUTPUT_PUBLISHED_0_1_0), Some(("testoutput","published")));
//         
//         assert_eq!(extract_subject_predicate(TESTSUITERUN_FINISHED_0_1_0), Some(("testsuiterun","finished")));
//         
//         assert_eq!(extract_subject_predicate(TESTSUITERUN_QUEUED_0_1_0), Some(("testsuiterun","queued")));
//         
//         assert_eq!(extract_subject_predicate(TESTSUITERUN_STARTED_0_1_0), Some(("testsuiterun","started")));
//         
//     }
// }