use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::PathBuf,
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Template {
    pub source: String,
    pub name: String,
    pub description: String,
    pub aliases: BTreeSet<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bucket {
    pub source: String,
    pub name: String,
    pub templates: BTreeMap<String, Template>,
    pub aliases: BTreeMap<String, Template>,
}

impl Bucket {
    pub fn new(name: String, source: String) -> Self {
        Self {
            source,
            name,
            templates: BTreeMap::new(),
            aliases: BTreeMap::new(),
        }
    }

    pub fn index_aliases(&mut self) {
        self.aliases.clear();
        self.templates.iter().for_each(|(name, template)| {
            self.aliases.insert(name.to_string(), template.clone());
            template.aliases.iter().for_each(|alias| {
                // TODO: Handle duplicate aliases
                self.aliases.insert(alias.to_string(), template.clone());
            });
        });
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OriConfig {
    pub buckets: BTreeMap<String, Bucket>,
}

#[derive(Debug)]
pub struct Configs {
    pub root_config: OriConfig,
    root_config_path: PathBuf,
}

impl Configs {
    pub fn new() -> Result<Self> {
        let root_config_partial_path = ".ori/config.json";
        let local_bucket_partial_path = ".ori/local";

        let home_dir = dirs::home_dir().context("Unable to get home directory")?;
        let root_config_path = std::path::Path::new(&home_dir).join(root_config_partial_path);
        let local_bucket_path = std::path::Path::new(&home_dir).join(local_bucket_partial_path);

        if let Ok(mut file) = File::open(&root_config_path) {
            let mut serialized_config = vec![];
            file.read_to_end(&mut serialized_config)?;

            let mut root_config: OriConfig = serde_json::from_slice(&serialized_config)?;
            root_config
                .buckets
                .entry("local".to_string())
                .or_insert_with(|| {
                    Bucket::new(
                        "local".to_string(),
                        local_bucket_path.to_str().unwrap().to_string(),
                    )
                });

            let config = Self {
                root_config,
                root_config_path,
            };

            config.write()?;

            return Ok(config);
        }

        Ok(Self {
            root_config_path,
            root_config: OriConfig {
                buckets: {
                    let mut buckets = BTreeMap::new();
                    buckets.insert(
                        "local".to_string(),
                        Bucket::new(
                            "local".to_string(),
                            local_bucket_path.to_str().unwrap().to_string(),
                        ),
                    );
                    buckets
                },
            },
        })
    }

    pub fn get_local_bucket(&mut self) -> &mut Bucket {
        self.root_config.buckets.get_mut("local").unwrap()
    }

    pub fn index_all_aliases(&mut self) {
        self.root_config
            .buckets
            .iter_mut()
            .for_each(|(_, bucket)| bucket.index_aliases());
    }

    pub fn write(&self) -> Result<()> {
        create_dir_all(self.root_config_path.parent().unwrap())?;
        let mut file = File::create(&self.root_config_path)?;
        let serialized_config = serde_json::to_vec_pretty(&self.root_config)?;
        file.write_all(&mut serialized_config.as_slice())?;
        file.sync_all()?;
        Ok(())
    }
}
