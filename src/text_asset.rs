use bevy::{
    asset::AssetLoader, prelude::*
};

#[derive(Asset, TypePath)]
pub struct TextAsset(pub String);

#[derive(Default, TypePath)]
pub struct TextAssetLoader;

impl AssetLoader for TextAssetLoader {
    type Asset = TextAsset;
    type Settings = ();
    type Error = std::io::Error;

    async fn load(
            &self,
            reader: &mut dyn bevy::asset::io::Reader,
            _settings: &Self::Settings,
            _load_context: &mut bevy::asset::LoadContext<'_>,
        ) -> Result<TextAsset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let text = String::from_utf8(bytes).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        Ok(TextAsset(text))
    }

    fn extensions(&self) -> &[&str] {
        &["txt"]
    }
}