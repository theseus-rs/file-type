use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28791369: FileFormat = FileFormat {
    id: 28_791_369,
    source_type: SourceType::Wikidata,
    name: "App Installer package",
    extensions: &["appx", "appxbundle"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
