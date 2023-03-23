use anyhow::anyhow;
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

use crate::article;

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);
    let watcher = RecommendedWatcher::new(move |res| {
        futures::executor::block_on(async {
            tx.send(res).await.unwrap();
        })
    }, Config::default())?;
    Ok((watcher, rx))
}

pub async fn async_watch<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;
    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => match event_handler(event) {
                Ok(_) => (),
                Err(e) => return Err(anyhow!(e)),
            },
            Err(e) => return Err(anyhow!(e)),
        }
    }
    Ok(())
}

fn event_handler(event: Event) -> anyhow::Result<()> {
    debug!("{:?}", event);
    match event.kind {
        // Create
        EventKind::Create(CreateKind::File) => {
            debug!("Create !");
            // TODO: Check title dup
            let md_path = event.paths.first().unwrap();
            match article::importer(&md_path) {
                Err(e) => return Err(e),
                Ok(_) => return Ok(()),
            };
        },
        // Modify
        EventKind::Modify(ModifyKind::Data(DataChange::Any)) => {
            debug!("Modify !");
            let md_path = event.paths.first().unwrap();
            match article::updater(&md_path) {
                Err(e) => return Err(e),
                Ok(_) => return Ok(()),
            };
        },
        // Delete
        EventKind::Remove(RemoveKind::File) => {
            debug!("Delete !");
            // TODO: Delete article on the database
        }
        _ => (),
    };
    Ok(())
}

