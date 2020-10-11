use crate::messages::register::Register;
use brcode::{BrCode, Label, Info, MerchantInfo};
use crate::actors::AsyncActor;
use actix::prelude::*;

pub struct CodeInfo {
    pub amount: Option<f64>,
    pub message: Option<String>,
    pub register: Register,
}


impl Message for CodeInfo {
    type Result = Result<String, ()>;
}

impl Handler<CodeInfo> for AsyncActor {
    type Result = Result<String, ()>;

    fn handle(&mut self, msg: CodeInfo, _: &mut Self::Context) -> Self::Result {
        let brcode = BrCode {
            payload_version: 1,
            initiation_method: None,
            merchant_category_code: 0000u32,
            merchant_name: msg.register.name,
            merchant_city: msg.register.city,
            merchant_information: vec![
                MerchantInfo {
                    id: 26,
                    info: vec![
                        Info {
                            id: 0,
                            info: "BR.GOV.BCB.PIX".to_string(),
                        },
                        Info {
                            id: 1,
                            info: msg.register.alias,
                        },
                        Info {
                            id: 2,
                            info: msg.message.unwrap_or("".to_string())
                        }
                    ],
                },
            ],
            currency: "986".to_string(),
            postal_code: Some(msg.register.postal_code),
            amount: msg.amount,
            country_code: "BR".to_string(),
            field_template: vec![Label {
                reference_label: "RP12345678-2019".to_string(),
            }],
            crc1610: "".to_string(),
            templates: None,
            merchant_account_information: None,
            convenience: None,
            convenience_fee_fixed: None,
            convenience_fee_percentage: None,
        };

        Ok(brcode::brcode_to_string(brcode))
    }
}
