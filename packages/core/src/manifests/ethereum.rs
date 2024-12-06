use bollard::models::PortBinding;
use eyre::Result;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::{
    domain::package::{Binding, Container, Package, PackageConfig, PackageDefinition},
    infra::file::kittynode_path,
};

pub(crate) struct Ethereum;

const ETHEREUM_NAME: &str = "Ethereum";

impl PackageDefinition for Ethereum {
    const NAME: &'static str = ETHEREUM_NAME;

    fn get_package() -> Result<Package> {
        let kittynode_path = kittynode_path()?;
        let jwt_path = kittynode_path.join("jwt.hex");

        let mut default_config = PackageConfig::new();
        default_config
            .values
            .insert("network".to_string(), "holesky".to_string());

        Ok(Package {
            name: ETHEREUM_NAME.to_string(),
            description: "This package installs a Reth execution client and a Lighthouse consensus client with Docker."
                .to_string(),
            network_name: "ethereum-network".to_string(),
            default_config,
            containers: Ethereum::get_containers(&jwt_path, "holesky")?,
        })
    }
}

impl Ethereum {
    pub(crate) fn get_containers(jwt_path: &PathBuf, network: &str) -> Result<Vec<Container>> {
        let checkpoint_sync_url = if network == "mainnet" {
            "https://mainnet.checkpoint.sigp.io/"
        } else {
            "https://checkpoint-sync.holesky.ethpandaops.io"
        };

        Ok(vec![
            Container {
                name: "reth-node".to_string(),
                image: "ghcr.io/paradigmxyz/reth".to_string(),
                cmd: vec![
                    "node".to_string(),
                    "--chain".to_string(),
                    network.to_string(),
                    "--metrics".to_string(),
                    "0.0.0.0:9001".to_string(),
                    "--authrpc.addr".to_string(),
                    "0.0.0.0".to_string(),
                    "--authrpc.port".to_string(),
                    "8551".to_string(),
                ],
                port_bindings: HashMap::from([
                    (
                        "9001/tcp".to_string(),
                        vec![PortBinding {
                            host_ip: Some("0.0.0.0".to_string()),
                            host_port: Some("9001".to_string()),
                        }],
                    ),
                    (
                        "30303/tcp".to_string(),
                        vec![PortBinding {
                            host_ip: Some("0.0.0.0".to_string()),
                            host_port: Some("30303".to_string()),
                        }],
                    ),
                    (
                        "30303/udp".to_string(),
                        vec![PortBinding {
                            host_ip: Some("0.0.0.0".to_string()),
                            host_port: Some("30303".to_string()),
                        }],
                    ),
                ]),
                volume_bindings: vec![Binding {
                    source: "rethdata".to_string(),
                    destination: format!("/root/.local/share/reth/{}", network),
                    options: None,
                }],
                file_bindings: vec![Binding {
                    source: jwt_path.display().to_string(),
                    destination: format!("/root/.local/share/reth/{}/jwt.hex", network),
                    options: Some("ro".to_string()),
                }],
            },
            Container {
                name: "lighthouse-node".to_string(),
                image: "sigp/lighthouse".to_string(),
                cmd: vec![
                    "lighthouse".to_string(),
                    "--network".to_string(),
                    network.to_string(),
                    "bn".to_string(),
                    "--execution-endpoint".to_string(),
                    "http://reth-node:8551".to_string(),
                    "--execution-jwt".to_string(),
                    "/jwt.hex".to_string(),
                    "--checkpoint-sync-url".to_string(),
                    checkpoint_sync_url.to_string(),
                ],
                port_bindings: HashMap::from([
                    (
                        "9000/tcp".to_string(),
                        vec![PortBinding {
                            host_ip: Some("0.0.0.0".to_string()),
                            host_port: Some("9000".to_string()),
                        }],
                    ),
                    (
                        "9000/udp".to_string(),
                        vec![PortBinding {
                            host_ip: Some("0.0.0.0".to_string()),
                            host_port: Some("9000".to_string()),
                        }],
                    ),
                    (
                        "9001/udp".to_string(),
                        vec![PortBinding {
                            host_ip: Some("0.0.0.0".to_string()),
                            host_port: Some("9001".to_string()),
                        }],
                    ),
                    (
                        "5052/tcp".to_string(),
                        vec![PortBinding {
                            host_ip: Some("127.0.0.1".to_string()),
                            host_port: Some("5052".to_string()),
                        }],
                    ),
                ]),
                volume_bindings: vec![],
                file_bindings: vec![
                    Binding {
                        source: jwt_path
                            .parent()
                            .unwrap()
                            .join(".lighthouse")
                            .to_string_lossy()
                            .to_string(),
                        destination: "/root/.lighthouse".to_string(),
                        options: None,
                    },
                    Binding {
                        source: jwt_path.to_string_lossy().to_string(),
                        destination: format!("/root/.lighthouse/{}/jwt.hex", network),
                        options: Some("ro".to_string()),
                    },
                ],
            },
        ])
    }
}
