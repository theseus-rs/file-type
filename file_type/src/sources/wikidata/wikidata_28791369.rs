use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28791369: FileFormat = FileFormat {
    id: 28_791_369,
    source_type: SourceType::Wikidata,
    name: "App Installer package",
    extensions: &["appx", "appxbundle"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
