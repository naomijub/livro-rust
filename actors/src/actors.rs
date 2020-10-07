use actix::{Actor, Context, SyncContext};

pub struct SyncActor;

impl Actor for SyncActor {
    type Context = SyncContext<Self>;
}

pub struct AsyncActor;

impl Actor for AsyncActor {
    type Context = Context<Self>;
}
