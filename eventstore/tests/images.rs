#![allow(dead_code)]

use std::collections::HashMap;
use testcontainers::{Container, Docker, Image, WaitForMessage};

const CONTAINER_IDENTIFIER: &str = "ghcr.io/eventstore/eventstore/eventstore";
const DEFAULT_TAG: &str = "ci";

#[derive(Debug, Default, Clone)]
pub struct ESDBArgs;

impl IntoIterator for ESDBArgs {
    type Item = String;
    type IntoIter = ::std::vec::IntoIter<String>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        vec![].into_iter()
    }
}

#[derive(Debug, Clone)]
pub struct ESDB {
    tag: String,
    arguments: ESDBArgs,
    env_vars: HashMap<String, String>,
    vol_vars: HashMap<String, String>,
}

impl ESDB {
    pub fn insecure_mode(mut self) -> Self {
        self.env_vars
            .insert("EVENTSTORE_INSECURE".to_string(), "true".to_string());
        self.env_vars.insert(
            "EVENTSTORE_ENABLE_ATOM_PUB_OVER_HTTP".to_string(),
            "true".to_string(),
        );

        self
    }

    pub fn secure_mode(mut self, is_secure: bool) -> Self {
        if is_secure {
            self.verify_certificates_exist().unwrap();

            self.env_vars.insert(
                "EVENTSTORE_CERTIFICATE_FILE".to_string(),
                "/etc/eventstore/certs/node/node.crt".to_string(),
            );

            self.env_vars.insert(
                "EVENTSTORE_CERTIFICATE_PRIVATE_KEY_FILE".to_string(),
                "/etc/eventstore/certs/node/node.key".to_string(),
            );

            self.env_vars.insert(
                "EVENTSTORE_TRUSTED_ROOT_CERTIFICATES_PATH".to_string(),
                "/etc/eventstore/certs/ca".to_string(),
            );

            let mut certs = std::env::current_dir().unwrap();
            certs.push("certs");

            self.vol_vars.insert(
                certs.as_path().display().to_string(),
                "/etc/eventstore/certs".to_string(),
            );

            self
        } else {
            self.insecure_mode()
        }
    }

    pub fn enable_projections(mut self) -> Self {
        self.env_vars
            .insert("EVENTSTORE_RUN_PROJECTIONS".to_string(), "all".to_string());
        self.env_vars.insert(
            "EVENTSTORE_START_STANDARD_PROJECTIONS".to_string(),
            "true".to_string(),
        );

        self
    }

    pub fn attach_volume_to_db_directory(mut self, volume: String) -> Self {
        self.vol_vars
            .insert(volume, "/var/lib/eventstore".to_string());

        self
    }

    fn verify_certificates_exist(&self) -> std::io::Result<()> {
        let mut root_dir = std::env::current_dir()?;
        let certs = &[
            ["ca", "ca.crt"],
            ["ca", "ca.key"],
            ["node", "node.crt"],
            ["node", "node.key"],
        ];

        root_dir.push("certs");

        for paths in certs {
            let mut tmp = root_dir.clone();

            for path in paths {
                tmp.push(path);
            }

            if !tmp.as_path().exists() {
                panic!("certificates directory is not configured properly, please run 'docker-compose --file configure-tls-for-tests.yml up'");
            }
        }

        Ok(())
    }
}

impl Image for ESDB {
    type Args = ESDBArgs;
    type EnvVars = HashMap<String, String>;
    type Volumes = HashMap<String, String>;
    type EntryPoint = std::convert::Infallible;

    fn descriptor(&self) -> String {
        format!("{}:{}", CONTAINER_IDENTIFIER, &self.tag)
    }

    fn wait_until_ready<D: Docker>(&self, container: &Container<'_, D, Self>) {
        container.logs().stdout.wait_for_message("SPARTA!").unwrap();
    }

    fn args(&self) -> Self::Args {
        self.arguments.clone()
    }

    fn env_vars(&self) -> Self::EnvVars {
        self.env_vars.clone()
    }

    fn volumes(&self) -> Self::Volumes {
        self.vol_vars.clone()
    }

    fn with_args(self, arguments: Self::Args) -> Self {
        ESDB { arguments, ..self }
    }
}

impl Default for ESDB {
    fn default() -> Self {
        let tag = option_env!("CONTAINER_IMAGE_VERSION").unwrap_or(DEFAULT_TAG);
        let mut env_vars = HashMap::new();

        env_vars.insert(
            "EVENTSTORE_GOSSIP_ON_SINGLE_NODE".to_string(),
            "true".to_string(),
        );

        ESDB {
            tag: tag.to_string(),
            arguments: ESDBArgs::default(),
            env_vars,
            vol_vars: HashMap::new(),
        }
    }
}
