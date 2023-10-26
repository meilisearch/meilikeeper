use meilikeeper::Zookeeper;

fn main() -> anyhoe::Result<()> {
    let mut zk = Zookeeper::connect("127.0.0.1:2181")?;

    for event_result in zk.watch("/keys") {
        match event_result? {
            Event::NodeCreated => todo!(),
            Event::NodeCreated => todo!(),
        }
    }

    Ok(())
}
