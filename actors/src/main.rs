use actix::prelude::*;

mod actors;
mod messages;
mod io;

use actors::{AsyncActor, SyncActor};
use messages::cep::CEP;
use messages::code_info::CodeInfo;
use messages::entity::Entity;
use messages::register::Register;

#[actix_rt::main]
async fn main() {
    let cpf = "05409000";
    let alias = "11999887766";
    let addr_sync = SyncArbiter::start(3, || SyncActor);
    let addr_async = AsyncActor {}.start();

    let city = addr_sync.send(CEP(cpf.to_string())).await.unwrap().unwrap();

    let _ = addr_sync
        .send(Register::new("julia", alias, cpf, &city))
        .await
        .unwrap();

    let entity = addr_async.send(Entity(alias.to_string())).await.unwrap();

    let amount = 45.59;
    let message = "Hello World";

    let brcode = addr_async.send(CodeInfo{
            amount: Some(amount),
            message: Some(message.to_string()), 
            register: entity.unwrap()
        })
        .await.unwrap();

    io::generate_qrcode(brcode.unwrap()).await;

    System::current().stop();
}
