mod node;
use ethers::prelude::*;
use eyre::Result;
use node::Node;
use rand::{prelude::ThreadRng, thread_rng, Rng};
use std::fmt;
use std::fs::File;
use std::io::Write;

abigen!(MockGobbler, "./out/MockGobbler.sol/MockGobbler.json");

const SAMPLES: u64 = 40;

pub struct Gobbler {
    pub node: Node,
    pub contract: MockGobbler<SignerMiddleware<Provider<Http>, LocalWallet>>,
    revealed_all: bool,
    rng: ThreadRng,
}

struct Shuffle(Vec<u64>);
impl fmt::Display for Shuffle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut comma_seperated = String::new();
        let vec: &Vec<u64> = &self.0;
        for num in &vec[0..vec.len() - 1] {
            comma_seperated.push_str(&num.to_string());
            comma_seperated.push_str(", ");
        }
        // TODO what about len == 0?
        comma_seperated.push_str(&vec[vec.len() - 1].to_string());

        write!(f, "{}", comma_seperated)
    }
}

impl Gobbler {
    async fn new() -> Result<Self> {
        let node = Node::new().await?;
        let contract = MockGobbler::deploy(node.client.clone(), ())?.send().await?;
        let revealed_all = false;
        let rng = thread_rng();

        Ok(Gobbler {
            node,
            contract,
            revealed_all,
            rng,
        })
    }

    async fn reveal_all(&mut self) -> Result<Shuffle> {
        // Can only reveal once
        assert!(!self.revealed_all);
        self.revealed_all = true;
        let num: u64 = self.rng.gen();
        let _ = self.contract.set_to_be_revealed(num).send().await?.await?;

        self.contract
            .reveal_gobblers(U256::from(9990))
            .send()
            .await?
            .await?;

        let shuffle: Vec<u64> = self
            .contract
            .get_all_gobbler_data()
            .call()
            .await?
            .map(|x| x.low_u64())
            .into();

        Ok(Shuffle(shuffle))
    }

    async fn reset(&mut self) -> Result<()> {
        // TODO why isn't resetting to old snapshot not working?
        // After 2 full iterations, I get a modulo by zero error from the node!
        self.node.reset(None).await?;
        let contract = MockGobbler::deploy(self.node.client.clone(), ())?
            .send()
            .await?;
        self.contract = contract;
        self.revealed_all = false;

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut ofile = File::options().append(true).open("./data/shuffles.csv")?;

    let mut gobbler = Gobbler::new().await?;
    for sample in 0..SAMPLES {
        println!("At iteration: {}", sample);
        let shuffle: Shuffle = gobbler.reveal_all().await?;
        gobbler.reset().await?;
        writeln!(ofile, "{}", shuffle)?;
    }

    Ok(())
}
