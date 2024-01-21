use std::error::Error;

use cdevents_sdk::{CDEvent, Context, Subject, pipelinerun_finished, Content};
use cloudevents::{Event, AttributesReader};
use time::OffsetDateTime;

fn main() -> Result<(), Box<dyn Error>> {
    let cdevent = CDEvent {
        context: Context {
            version: "0.3.0".to_string(),
            id: "271069a8-fc18-44f1-b38f-9d70a1695819".to_string(),
            r#type: "dev.cdevents.pipelinerun .finished.0.1.1".to_string(),
            source: "/event/source/123".try_into()?,
            timestamp: OffsetDateTime::now_utc(),
        },
        subject: Subject {
            id: "/dev/pipeline/run/1".to_string(),
            source: Some("https://dev.pipeline.run/source".try_into()?),
            r#type: "build".to_string(),
            content: Content::PipelinerunFinished(pipelinerun_finished::Content{
                errors: Some("pipelineErrors".into()),
                outcome: Some("success".into()),
                pipeline_name: Some("testPipeline".into()),
                url: Some("https://dev.pipeline.run/source".into())
            })
        },
        custom_data: None,
        custom_data_content_type: None,
    };

    let cdevent_expected = cdevent.clone();

    // shortcut for creating cloudevents with
    //
    // ```rust
    // use cloudevents::event::EventBuilderV10;
    // use cdevents_sdk::cloudevents::BuilderExt;
    //
    // let mut cloudevent = EventBuilderV10::new().with_cdevent(cdevent.clone())?.build()?;
    // ```
    let cloudevent: Event = cdevent.try_into()?;

    // zero transport, but cloning
    let cloudevent_received: Event = cloudevent.clone();
    let cdevent_extracted: CDEvent = cloudevent_received.try_into()?;

    assert_eq!(cloudevent.id(), cdevent_extracted.context.id);
    assert_eq!(cdevent_expected, cdevent_extracted);
    Ok(())
}
