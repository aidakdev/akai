use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct SuperflakeOptions {
    pub node_id: u16,
    pub time_offset: u64
}

impl Default for SuperflakeOptions {
    fn default() -> Self {
        SuperflakeOptions {
            node_id: 1,
            time_offset: 0
        }
    }
}

#[derive(Debug)]
pub struct Superflake {
    sequence: u16,
    last_time: u64,
    node_id: u16,
    time_offset: u64
}

impl Superflake {
    pub fn new(options: SuperflakeOptions) -> Self {
        Superflake {
            sequence: 0,
            last_time: 0,
            node_id: options.node_id % 1023,
            time_offset: options.time_offset
        }
    }

    pub fn gen(&mut self) -> u128 {
        let now_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards. What the fuck, man?")
            .as_millis() as u64;

        let gen_time = format!("{:b}", now_time - self.time_offset);

        self.sequence = 0;

        if self.last_time == now_time {
            self.sequence += 1;

            if self.sequence > 4095 {
                self.sequence = 0;

                while SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards. What the fuck, man, again?")
                    .as_millis() as u64
                    <= now_time
                {}
            }
        }

        self.last_time = now_time;

        let gen_sequence = format!("{:012b}", self.sequence);
        let gen_node = format!("{:010b}", self.node_id);
        let raw_id = format!("{}{}{}", gen_time, gen_node, gen_sequence);

        let mut id = 0u128;

        for i in (0..raw_id.len()).step_by(4).rev() {
            let chunk = &raw_id[i.saturating_sub(4)..i];

            let parsed_chunk = if chunk.is_empty() {
                0
            } else {
                u128::from_str_radix(chunk, 2).unwrap()
            };

            id = (id << 4) + parsed_chunk;
        }

        id
    }
}