use actix::prelude::*;

mod actors;
mod messages;

use actors::SyncActor;
use messages::cep::CEP;
use messages::register::Register;

#[actix_rt::main]
async fn main() {
    let cpf = "05409000";
    let addr_sync = SyncArbiter::start(3, || SyncActor);

    let city = addr_sync
        .send(CEP(cpf.to_string()))
        .await
        .unwrap()
        .unwrap();

    let register = addr_sync
        .send(Register::new("julia", "alias", cpf, &city))
        .await
        .unwrap();

    println!("{:?}", register);
    System::current().stop();
}