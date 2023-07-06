pub use bitcoin::Network as RustNetwork;

uniffi::include_scaffolding!("bitcoin_ffi");

pub enum Network {
    /// Mainnet Bitcoin.
    Bitcoin,
    /// Bitcoin's testnet network.
    Testnet,
    /// Bitcoin's signet network.
    Signet,
    /// Bitcoin's regtest network.
    Regtest,
}

impl From<Network> for RustNetwork {
    fn from(network: Network) -> Self {
        match network {
            Network::Bitcoin => RustNetwork::Bitcoin,
            Network::Testnet => RustNetwork::Testnet,
            Network::Signet  => RustNetwork::Signet,
            Network::Regtest => RustNetwork::Regtest,
        }
    }
}

impl From<RustNetwork> for Network {
    fn from(network: RustNetwork) -> Self {
        match network {
            RustNetwork::Bitcoin => Network::Bitcoin,
            RustNetwork::Testnet => Network::Testnet,
            RustNetwork::Signet  => Network::Signet,
            RustNetwork::Regtest => Network::Regtest,
            _                    => panic!("Network {} not supported", network),
        }
    }
}

#[cfg(test)]
mod tests {}
