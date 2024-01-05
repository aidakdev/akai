use hex::encode;
use rand::random;
use crate::superflake::{SuperflakeOptions, Superflake};

#[derive(Debug)]
pub struct AkaiPrefix {
    pub content: String,
    pub allows_lowercase: bool
}

#[derive(Debug)]
pub struct Akai {
    prefix: AkaiPrefix,
    custom_bytes: Option<usize>,
    custom_timestamp: Option<u64>
}

#[derive(Debug)]
pub struct AkaiOptions {
    pub prefix: AkaiPrefix,
    pub custom_bytes: Option<usize>,
    pub custom_timestamp: Option<u64>
}

const VALID_PREFIX: &str = r"^[a-z0-9_]+$";
const DEFAULT_TIMESTAMP: u64 = 1616275800; // March 20, 2021

impl Akai {
    pub fn new(
        options: AkaiOptions
    ) -> Self {
        Akai {
            prefix: options.prefix,
            custom_bytes: options.custom_bytes,
            custom_timestamp: options.custom_timestamp
        }
    }

    pub fn generate(&self) -> Result<String, &'static str> {
        if !self.is_valid_prefix() && !self.prefix.allows_lowercase {
            return Err(
                "Akai ID prefixes should be lowercase. To ignore this, make `allows_lowercase` true."
            );
        }

        let random = 
            encode(self.generate_random_bytes().as_slice());

        let superflake = 
            Superflake::new(SuperflakeOptions {
                node_id: 1,
                time_offset: self.custom_timestamp.unwrap_or(DEFAULT_TIMESTAMP)
            }).gen();

        Ok(
            format!(
                "{}_{}{}", 
                self.prefix.content, 
                superflake, 
                random
            )
        )
    }

    fn is_valid_prefix(&self) -> bool {
        regex::Regex::new(VALID_PREFIX)
            .unwrap()
            .is_match(&self.prefix.content)
    }

    fn generate_random_bytes(&self) -> Vec<u8> {
        let byte_count =
            self.custom_bytes.unwrap_or(8);

        (0..byte_count).map(|_| random::<u8>()).collect()
    }
}
