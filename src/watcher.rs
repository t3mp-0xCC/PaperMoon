use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use log::debug;
use notify::{
    event::{CreateKind, DataChange, ModifyKind, RemoveKind},
    Config, Event, EventKind, RecursiveMode,
    RecommendedWatcher, Watcher
};
use std::path::Path;

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);
    let watcher = RecommendedWatcher::new(move |res| {
        futures::executor::block_on(async {
            tx.send(res).await.unwrap();
        })
    }, Config::default())?;
    Ok((watcher, rx))
}

pub async fn async_watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;
    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => event_handler(event),
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

fn event_handler(event: Event) {
    debug!("{:?}", event);
    match event.kind {
        // Create
        EventKind::Create(CreateKind::File) => {
            debug!("Create !");
        },
        // Modify
        EventKind::Modify(ModifyKind::Data(DataChange::Any)) => {
            debug!("Modify !");
        },
        // Delete
        EventKind::Remove(RemoveKind::File) => {
            debug!("Delete !");
        }
        _ => (),
    };
}
