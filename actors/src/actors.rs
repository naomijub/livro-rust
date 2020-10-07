use actix::{Actor, SyncContext};

pub struct SyncActor;

impl Actor for SyncActor {
    type Context = SyncContext<Self>;
}
