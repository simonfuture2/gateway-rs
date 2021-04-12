use crate::*;
use helium_proto::BlockchainTxnAddGatewayV1;
use structopt::StructOpt;

/// Construct an add gateway transaction for this gateway.
#[derive(Debug, StructOpt)]
pub struct Cmd {
    owner: PublicKey,
    payer: PublicKey,
}

impl Cmd {
    pub async fn run(&self, settings: Settings) -> Result {
        let txn = BlockchainTxnAddGatewayV1 {
            gateway: settings.keypair.public_key.into(),
            owner: self.owner.into(),
            payer: self.payer.into(),
            owner_signature: vec![],
            gateway_signature: vec![],
            payer_signature: vec![],
        };
        println!("Txn: {:?}", txn);
        Ok(())
    }
}
