use std::error::Error;

use cdevents_sdk::{CDEvent, Subject, pipelinerun_finished};
use cloudevents::{Event, AttributesReader};

fn main() -> Result<(), Box<dyn Error>> {
    let cdevent = CDEvent::from(
        Subject::from(pipelinerun_finished::Content{
            errors: Some("pipelineErrors".into()),
            outcome: Some("success".into()),
            pipeline_name: Some("testPipeline".into()),
            url: Some("https://dev.pipeline.run/url".into())
        })
        .with_id("/dev/pipeline/run/1".try_into()?)
        .with_source("https://dev.pipeline.run/source".try_into()?)
    )
    .with_id("271069a8-fc18-44f1-b38f-9d70a1695819".try_into()?)
    .with_source("https://dev.cdevents".try_into()?)
    ;

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

    assert_eq!(cloudevent.id(), cdevent_extracted.id().to_string());
    assert_eq!(cdevent_expected, cdevent_extracted);
    Ok(())
}

