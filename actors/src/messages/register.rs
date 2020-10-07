use crate::actors::SyncActor;
use actix::{Handler, Message};
use edn_derive::Serialize;
use edn_rs::{Deserialize, EdnError};
use transistor::client::Crux;
use transistor::types::Action;
use transistor::types::CruxId;

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct Register {
    crux__db___id: CruxId,
    name: String,
    alias: String,
    postal_code: String,
    city: String,
}

impl Deserialize for Register {
    fn deserialize(edn: &edn_rs::Edn) -> Result<Self, EdnError> {
        Ok(Self {
            crux__db___id: edn_rs::from_edn(&edn[":crux.db/id"])?,
            name: edn_rs::from_edn(&edn[":name"])?,
            alias: edn_rs::from_edn(&edn[":alias"])?,
            city: edn_rs::from_edn(&edn[":city"])?,
            postal_code: edn_rs::from_edn(&edn[":postal-code"])?,
        })
    }
}

impl Register {
    pub fn new(name: &str, alias: &str, cep: &str, city: &str) -> Register {
        Register {
            crux__db___id: CruxId::new(alias),
            name: name.to_string(),
            alias: alias.to_string(),
            postal_code: cep.to_string(),
            city: city.to_string(),
        }
    }
}

impl Message for Register {
    type Result = Result<bool, ()>;
}

impl Handler<Register> for SyncActor {
    type Result = Result<bool, ()>;

    fn handle(&mut self, msg: Register, _: &mut Self::Context) -> Self::Result {
        let client = Crux::new("127.0.0.1", "3000").http_client();
        let action = Action::Put(edn_rs::to_string(msg), None);
        let tx_log = client.tx_log(vec![action]);
        match tx_log {
            Ok(tx) => {
                println!("{:?}", tx);
                Ok(true)
            }
            _ => Err(()),
        }
    }
}
