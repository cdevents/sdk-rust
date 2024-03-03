use cloudevents::{Data, Event};
use time::format_description::well_known::Rfc3339;

use crate::CDEvent;

pub trait BuilderExt: Sized {
    type Error;
    fn with_cdevent(self, cdevent: CDEvent) -> Result<Self, Self::Error>;
}

impl BuilderExt for cloudevents::EventBuilderV10 {
    type Error = cloudevents::message::Error;

    fn with_cdevent(self, cdevent: CDEvent) -> Result<Self, Self::Error> {
        Ok(
            self.id(cdevent.id())
                .ty(cdevent.ty())
                .source(cdevent.source().as_str())
                .subject(cdevent.subject().id())
                .time(cdevent.timestamp().format(&Rfc3339).map_err(|e| Self::Error::Other{source: Box::new(e)})?)
                .data("application/json", serde_json::to_value(cdevent).map_err(Self::Error::from)?)
        )
    }
}

impl TryFrom<CDEvent> for Event {
    type Error = cloudevents::message::Error;

    fn try_from(value: CDEvent) -> Result<Self, Self::Error> {
        use ::cloudevents::{EventBuilder, EventBuilderV10};
        EventBuilderV10::new()
            .with_cdevent(value)?
            .build().map_err(Self::Error::from)
    }
}

impl TryFrom<Data> for CDEvent {
    type Error = serde_json::Error;

    fn try_from(value: Data) -> Result<Self, Self::Error> {
        let json = match value {
            Data::Binary(v) => serde_json::from_slice(&v)?,
            Data::Json(v) => v,
            Data::String(s) => serde_json::from_str(&s)?,
        };
        serde_json::from_value(json) //doesn't work due to the unsupported variant definition
    }
}

impl TryFrom<Event> for CDEvent {
    type Error = crate::Error;

    fn try_from(value: Event) -> Result<Self, Self::Error> {
        let mut event = value;
        let (_, _, data) = event.take_data();
        data.ok_or(Self::Error::DataNotFoundInCloudEvent)?
            .try_into().map_err(Self::Error::from)
    }
}

#[cfg(test)]
mod tests {
    use ::cloudevents::{AttributesReader, EventBuilder, EventBuilderV10};
    use crate::*;
    use super::*;

    #[test]
    fn test_into_cloudevent() -> Result<(), Box<dyn std::error::Error>> {
        let cdevent = CDEvent::from(
            Subject::from(latest::build_queued::Content{})
                .with_id("subject123".try_into()?)
                .with_source("/event/source/123".try_into()?)
        )
        .with_id("271069a8-fc18-44f1-b38f-9d70a1695819".try_into()?)
        .with_source("https://dev.cdevents".try_into()?)
        ;

        let cloudevent_via_builder = EventBuilderV10::new()
            .with_cdevent(cdevent.clone())?
            .build()?;
        let mut cloudevent: Event = cdevent.clone().try_into()?;
        assert_eq!(cloudevent_via_builder, cloudevent);

        assert_eq!(cloudevent.id(), "271069a8-fc18-44f1-b38f-9d70a1695819");
        assert_eq!(cloudevent.id(), cdevent.id().to_string());

        let (_, _, data) = cloudevent.take_data();
        let cdevent_extracted: CDEvent = data.ok_or(Error::DataNotFoundInCloudEvent)?.try_into()?;
        assert_eq!(cloudevent.id(), cdevent_extracted.id().to_string());
        assert_eq!(cdevent, cdevent_extracted);
        Ok(())
    }
}
