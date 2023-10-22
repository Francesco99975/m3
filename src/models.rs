#[derive(clap::ValueEnum, Clone, Default, Debug)]
pub enum ModLoader {
    #[default]
    Fabric,
    Forge,
}

#[derive(clap::ValueEnum, Clone, Default, Debug)]
pub enum VersionChannel {
    #[default]
    Release,
    Beta,
    Alpha,
}
