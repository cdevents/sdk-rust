# Rust CDEvents SDK

Rust SDK to emit [CDEvents](https://cdevents.dev).

The SDK can be used to create CDEvents and send them as CloudEvents, as well as parse a received CloudEvent into a CDEvent.

## Create and send your first CDEvent as CloudEvent

Import the modules in your code

```toml
cdevents-sdk = "0.1"
```

To send a CDEvent as CloudEvent:

```rust
// from examples/pipelinerun_finished.rs
use std::error::Error;

use cdevents_sdk::{CDEvent, Subject, spec_0_3_0::pipelinerun_finished, Content};
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
```

See the [CloudEvents](https://github.com/cloudevents/sdk-rust) docs as well.

## Features

- [x] support cdevents spec 0.3.0
- [x] support cdevents spec 0.4.1
- [ ] support of custom event
  - [ ] compile-time generation of type for custom event
  - [ ] runtime validation (download of jsonschemas & validation)
  - [x] serialize/deserialize of custom event (type `dev.cdeventsx.{subject}.{predicate}.{version}`)
  - [x] partial validation of custom event (`subject.content` is not validated, stored as json)
- [x] Cloudevents support (provide wrapper/extractor for CDEvent)
- [x] CDEvent stored into static types (=> no use of jsonshema at runtime)
- [x] rutime validation for scalar types (purl, datetime, uri-reference)
- [ ] report clear and readable error messages on deserialization
- [x] provide type with builder pattern (fluent)
- [x] provide random/sample generator for CDEvent (property based testing)
- [x] test `serialization(deserialization(of spec's examples & conformances)) == spec's examples & conformances`
- [x] test `serialization(random CDEvent) matches the jsonschemas`

## References

- [CDEvents](https://cdevents.dev)
- [CDEvents Primer](https://cdevents.dev/docs/primer/)
- [CDEvents Specification](https://cdevents.dev/docs/)
