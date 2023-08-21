use clap::ValueEnum;
use rand::Rng;

#[derive(Debug, Clone, ValueEnum, Default)]
pub enum IpAddrFormat {
    #[default]
    Ipv4,
    Ipv6,
}

/// Generates a string representation of a random IP address in the specified format.
///
/// Note: the generated addresses are guaranteed to be *structurally* correct, but are *not*
/// guaranteed to be semantically correct.
pub fn gen_ip_addr_string(fmt: &IpAddrFormat) -> String {
    let mut rng = rand::thread_rng();

    match fmt {
        IpAddrFormat::Ipv4 => {
            let num_groups = 4;
            let mut groups = Vec::with_capacity(num_groups);

            for _ in 0..num_groups {
                let group = rng.gen_range(u8::MIN..=u8::MAX);
                groups.push(group.to_string())
            }

            groups.join(".")
        }
        IpAddrFormat::Ipv6 => {
            let num_groups = 8;
            let mut groups = Vec::with_capacity(num_groups);

            for _ in 0..num_groups {
                let group = rng.gen_range(u16::MIN..=u16::MAX);
                groups.push(format!("{group:04x}"))
            }

            groups.join(":")
        }
    }
}
