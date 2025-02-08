#[test]
fn test_concurrent_input_resolution() {
    let mut sync = Synchronizer::new();
    
    let event1 = TimedEvent::new(Event::MouseMove { x: 100, y: 200 }, 1);
    let event2 = TimedEvent::new(Event::MouseMove { x: 150, y: 250 }, 1);
    
    sync.add_event(event1.clone(), DeviceId::A);
    sync.add_event(event2.clone(), DeviceId::B);
    
    let resolved = sync.resolve_conflicts();
    
    // Should keep last write based on vector clock
    assert!(resolved.contains(&event2));
    assert!(!resolved.contains(&event1));
} 

pub trait EventCodec: Send + Sync {
    fn encode(&self, event: &Event) -> Result<Vec<u8>>;
    fn decode(&self, data: &[u8]) -> Result<Event>;
}

// JSON implementation
pub struct JsonCodec;

impl EventCodec for JsonCodec {
    fn encode(&self, event: &Event) -> Result<Vec<u8>> {
        serde_json::to_vec(event).map_err(Into::into)
    }

    fn decode(&self, data: &[u8]) -> Result<Event> {
        serde_json::from_slice(data).map_err(Into::into)
    }
}

// Later add: 
// pub struct CborCodec;
// pub struct BincodeCodec; 

pub struct NetworkConnection<C: EventCodec = JsonCodec> {
    reader: BufReader<OwnedReadHalf>,
    writer: OwnedWriteHalf,
    codec: C,
}

impl<C: EventCodec> NetworkConnection<C> {
    pub fn new(stream: TcpStream, codec: C) -> Self {
        let (reader_half, writer_half) = stream.into_split();
        Self {
            reader: BufReader::new(reader_half),
            writer: writer_half,
            codec,
        }
    }

    pub async fn send_event(&mut self, event: &Event) -> Result<()> {
        let data = self.codec.encode(event)?;
        self.writer.write_all(&data).await?;
        self.writer.write_all(b"\n").await?;  // Keep delimiter for now
        Ok(())
    }

    pub async fn receive_event(&mut self) -> Result<Event> {
        let mut line = String::new();
        self.reader.read_line(&mut line).await?;
        self.codec.decode(line.trim().as_bytes())
    }
} 