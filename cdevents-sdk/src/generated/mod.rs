// code generated by cdevents/sdk-rust/generator (mod.hbs)
pub mod artifact_packaged_subject;
pub mod artifact_published_subject;
pub mod artifact_signed_subject;
pub mod branch_created_subject;
pub mod branch_deleted_subject;
pub mod build_finished_subject;
pub mod build_queued_subject;
pub mod build_started_subject;
pub mod change_abandoned_subject;
pub mod change_created_subject;
pub mod change_merged_subject;
pub mod change_reviewed_subject;
pub mod change_updated_subject;
pub mod environment_created_subject;
pub mod environment_deleted_subject;
pub mod environment_modified_subject;
pub mod incident_detected_subject;
pub mod incident_reported_subject;
pub mod incident_resolved_subject;
pub mod pipeline_run_finished_subject;
pub mod pipeline_run_queued_subject;
pub mod pipeline_run_started_subject;
pub mod repository_created_subject;
pub mod repository_deleted_subject;
pub mod repository_modified_subject;
pub mod service_deployed_subject;
pub mod service_published_subject;
pub mod service_removed_subject;
pub mod service_rolledback_subject;
pub mod service_upgraded_subject;
pub mod task_run_finished_subject;
pub mod task_run_started_subject;
pub mod test_case_run_finished_subject;
pub mod test_case_run_queued_subject;
pub mod test_case_run_started_subject;
pub mod test_output_published_subject;
pub mod test_suite_finished_subject;
pub mod test_suite_run_queued_subject;
pub mod test_suite_run_started_subject;

use serde::{Serialize, Deserialize};

#[derive(Debug,Clone,Serialize,Deserialize, PartialEq, Eq)]
#[serde(untagged)] // TODO how to use content of context.type as discriminator ?
pub enum Subject {
    ArtifactPackagedSubject(artifact_packaged_subject::Subject),
    ArtifactPublishedSubject(artifact_published_subject::Subject),
    ArtifactSignedSubject(artifact_signed_subject::Subject),
    BranchCreatedSubject(branch_created_subject::Subject),
    BranchDeletedSubject(branch_deleted_subject::Subject),
    BuildFinishedSubject(build_finished_subject::Subject),
    BuildQueuedSubject(build_queued_subject::Subject),
    BuildStartedSubject(build_started_subject::Subject),
    ChangeAbandonedSubject(change_abandoned_subject::Subject),
    ChangeCreatedSubject(change_created_subject::Subject),
    ChangeMergedSubject(change_merged_subject::Subject),
    ChangeReviewedSubject(change_reviewed_subject::Subject),
    ChangeUpdatedSubject(change_updated_subject::Subject),
    EnvironmentCreatedSubject(environment_created_subject::Subject),
    EnvironmentDeletedSubject(environment_deleted_subject::Subject),
    EnvironmentModifiedSubject(environment_modified_subject::Subject),
    IncidentDetectedSubject(incident_detected_subject::Subject),
    IncidentReportedSubject(incident_reported_subject::Subject),
    IncidentResolvedSubject(incident_resolved_subject::Subject),
    PipelineRunFinishedSubject(pipeline_run_finished_subject::Subject),
    PipelineRunQueuedSubject(pipeline_run_queued_subject::Subject),
    PipelineRunStartedSubject(pipeline_run_started_subject::Subject),
    RepositoryCreatedSubject(repository_created_subject::Subject),
    RepositoryDeletedSubject(repository_deleted_subject::Subject),
    RepositoryModifiedSubject(repository_modified_subject::Subject),
    ServiceDeployedSubject(service_deployed_subject::Subject),
    ServicePublishedSubject(service_published_subject::Subject),
    ServiceRemovedSubject(service_removed_subject::Subject),
    ServiceRolledbackSubject(service_rolledback_subject::Subject),
    ServiceUpgradedSubject(service_upgraded_subject::Subject),
    TaskRunFinishedSubject(task_run_finished_subject::Subject),
    TaskRunStartedSubject(task_run_started_subject::Subject),
    TestCaseRunFinishedSubject(test_case_run_finished_subject::Subject),
    TestCaseRunQueuedSubject(test_case_run_queued_subject::Subject),
    TestCaseRunStartedSubject(test_case_run_started_subject::Subject),
    TestOutputPublishedSubject(test_output_published_subject::Subject),
    TestSuiteFinishedSubject(test_suite_finished_subject::Subject),
    TestSuiteRunQueuedSubject(test_suite_run_queued_subject::Subject),
    TestSuiteRunStartedSubject(test_suite_run_started_subject::Subject),
}

impl Subject {
    pub fn from_json(ty: &str, json: serde_json::Value) -> Self{
        match ty {
            "dev.cdevents.artifact.packaged.0.1.1" => {
                let subject: artifact_packaged_subject::Subject = serde_json::from_value(json).unwrap();
                Self::ArtifactPackagedSubject(subject)
            },
            "dev.cdevents.artifact.published.0.1.1" => {
                let subject: artifact_published_subject::Subject = serde_json::from_value(json).unwrap();
                Self::ArtifactPublishedSubject(subject)
            },
            "dev.cdevents.artifact.signed.0.1.0" => {
                let subject: artifact_signed_subject::Subject = serde_json::from_value(json).unwrap();
                Self::ArtifactSignedSubject(subject)
            },
            "dev.cdevents.branch.created.0.1.2" => {
                let subject: branch_created_subject::Subject = serde_json::from_value(json).unwrap();
                Self::BranchCreatedSubject(subject)
            },
            "dev.cdevents.branch.deleted.0.1.2" => {
                let subject: branch_deleted_subject::Subject = serde_json::from_value(json).unwrap();
                Self::BranchDeletedSubject(subject)
            },
            "dev.cdevents.build.finished.0.1.1" => {
                let subject: build_finished_subject::Subject = serde_json::from_value(json).unwrap();
                Self::BuildFinishedSubject(subject)
            },
            "dev.cdevents.build.queued.0.1.1" => {
                let subject: build_queued_subject::Subject = serde_json::from_value(json).unwrap();
                Self::BuildQueuedSubject(subject)
            },
            "dev.cdevents.build.started.0.1.1" => {
                let subject: build_started_subject::Subject = serde_json::from_value(json).unwrap();
                Self::BuildStartedSubject(subject)
            },
            "dev.cdevents.change.abandoned.0.1.2" => {
                let subject: change_abandoned_subject::Subject = serde_json::from_value(json).unwrap();
                Self::ChangeAbandonedSubject(subject)
            },
            "dev.cdevents.change.created.0.1.2" => {
                let subject: change_created_subject::Subject = serde_json::from_value(json).unwrap();
                Self::ChangeCreatedSubject(subject)
            },
            "dev.cdevents.change.merged.0.1.2" => {
                let subject: change_merged_subject::Subject = serde_json::from_value(json).unwrap();
                Self::ChangeMergedSubject(subject)
            },
            "dev.cdevents.change.reviewed.0.1.2" => {
                let subject: change_reviewed_subject::Subject = serde_json::from_value(json).unwrap();
                Self::ChangeReviewedSubject(subject)
            },
            "dev.cdevents.change.updated.0.1.2" => {
                let subject: change_updated_subject::Subject = serde_json::from_value(json).unwrap();
                Self::ChangeUpdatedSubject(subject)
            },
            "dev.cdevents.environment.created.0.1.1" => {
                let subject: environment_created_subject::Subject = serde_json::from_value(json).unwrap();
                Self::EnvironmentCreatedSubject(subject)
            },
            "dev.cdevents.environment.deleted.0.1.1" => {
                let subject: environment_deleted_subject::Subject = serde_json::from_value(json).unwrap();
                Self::EnvironmentDeletedSubject(subject)
            },
            "dev.cdevents.environment.modified.0.1.1" => {
                let subject: environment_modified_subject::Subject = serde_json::from_value(json).unwrap();
                Self::EnvironmentModifiedSubject(subject)
            },
            "dev.cdevents.incident.detected.0.1.0" => {
                let subject: incident_detected_subject::Subject = serde_json::from_value(json).unwrap();
                Self::IncidentDetectedSubject(subject)
            },
            "dev.cdevents.incident.reported.0.1.0" => {
                let subject: incident_reported_subject::Subject = serde_json::from_value(json).unwrap();
                Self::IncidentReportedSubject(subject)
            },
            "dev.cdevents.incident.resolved.0.1.0" => {
                let subject: incident_resolved_subject::Subject = serde_json::from_value(json).unwrap();
                Self::IncidentResolvedSubject(subject)
            },
            "dev.cdevents.pipelinerun.finished.0.1.1" => {
                let subject: pipeline_run_finished_subject::Subject = serde_json::from_value(json).unwrap();
                Self::PipelineRunFinishedSubject(subject)
            },
            "dev.cdevents.pipelinerun.queued.0.1.1" => {
                let subject: pipeline_run_queued_subject::Subject = serde_json::from_value(json).unwrap();
                Self::PipelineRunQueuedSubject(subject)
            },
            "dev.cdevents.pipelinerun.started.0.1.1" => {
                let subject: pipeline_run_started_subject::Subject = serde_json::from_value(json).unwrap();
                Self::PipelineRunStartedSubject(subject)
            },
            "dev.cdevents.repository.created.0.1.1" => {
                let subject: repository_created_subject::Subject = serde_json::from_value(json).unwrap();
                Self::RepositoryCreatedSubject(subject)
            },
            "dev.cdevents.repository.deleted.0.1.1" => {
                let subject: repository_deleted_subject::Subject = serde_json::from_value(json).unwrap();
                Self::RepositoryDeletedSubject(subject)
            },
            "dev.cdevents.repository.modified.0.1.1" => {
                let subject: repository_modified_subject::Subject = serde_json::from_value(json).unwrap();
                Self::RepositoryModifiedSubject(subject)
            },
            "dev.cdevents.service.deployed.0.1.1" => {
                let subject: service_deployed_subject::Subject = serde_json::from_value(json).unwrap();
                Self::ServiceDeployedSubject(subject)
            },
            "dev.cdevents.service.published.0.1.1" => {
                let subject: service_published_subject::Subject = serde_json::from_value(json).unwrap();
                Self::ServicePublishedSubject(subject)
            },
            "dev.cdevents.service.removed.0.1.1" => {
                let subject: service_removed_subject::Subject = serde_json::from_value(json).unwrap();
                Self::ServiceRemovedSubject(subject)
            },
            "dev.cdevents.service.rolledback.0.1.1" => {
                let subject: service_rolledback_subject::Subject = serde_json::from_value(json).unwrap();
                Self::ServiceRolledbackSubject(subject)
            },
            "dev.cdevents.service.upgraded.0.1.1" => {
                let subject: service_upgraded_subject::Subject = serde_json::from_value(json).unwrap();
                Self::ServiceUpgradedSubject(subject)
            },
            "dev.cdevents.taskrun.finished.0.1.1" => {
                let subject: task_run_finished_subject::Subject = serde_json::from_value(json).unwrap();
                Self::TaskRunFinishedSubject(subject)
            },
            "dev.cdevents.taskrun.started.0.1.1" => {
                let subject: task_run_started_subject::Subject = serde_json::from_value(json).unwrap();
                Self::TaskRunStartedSubject(subject)
            },
            "dev.cdevents.testcaserun.finished.0.1.0" => {
                let subject: test_case_run_finished_subject::Subject = serde_json::from_value(json).unwrap();
                Self::TestCaseRunFinishedSubject(subject)
            },
            "dev.cdevents.testcaserun.queued.0.1.0" => {
                let subject: test_case_run_queued_subject::Subject = serde_json::from_value(json).unwrap();
                Self::TestCaseRunQueuedSubject(subject)
            },
            "dev.cdevents.testcaserun.started.0.1.0" => {
                let subject: test_case_run_started_subject::Subject = serde_json::from_value(json).unwrap();
                Self::TestCaseRunStartedSubject(subject)
            },
            "dev.cdevents.testoutput.published.0.1.0" => {
                let subject: test_output_published_subject::Subject = serde_json::from_value(json).unwrap();
                Self::TestOutputPublishedSubject(subject)
            },
            "dev.cdevents.testsuiterun.finished.0.1.0" => {
                let subject: test_suite_finished_subject::Subject = serde_json::from_value(json).unwrap();
                Self::TestSuiteFinishedSubject(subject)
            },
            "dev.cdevents.testsuiterun.queued.0.1.0" => {
                let subject: test_suite_run_queued_subject::Subject = serde_json::from_value(json).unwrap();
                Self::TestSuiteRunQueuedSubject(subject)
            },
            "dev.cdevents.testsuiterun.started.0.1.0" => {
                let subject: test_suite_run_started_subject::Subject = serde_json::from_value(json).unwrap();
                Self::TestSuiteRunStartedSubject(subject)
            },
            x => todo!("no implementation for type '{}', please open an issue", x),
        }
    }
}