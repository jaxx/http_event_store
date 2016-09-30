use api::Api;
use Stream;
use event::Event;
use types::Result;

pub struct Client {
    api: Api
}

impl Client {
    pub fn new() -> Client {
        Client { api: Api {} }
    }

    pub fn append_to_stream<T: Event>(&self, stream_name: &str, expectedVersion: u64, events: Vec<Box<T>>) {
        self.api.append_to_stream(stream_name, expectedVersion, events)
    }

    // TODO Restrict `count` using u8 or u16
    pub fn read_stream_events_forward(&self, stream_name: &str, start: u32, count: u32, resolve_link_tos: bool) -> Result<Stream> {
        self.api.read_stream_events_forward(stream_name, start, count, resolve_link_tos)
    }
}
