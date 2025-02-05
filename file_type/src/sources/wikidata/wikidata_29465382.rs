use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29465382: FileFormat = FileFormat {
    id: 29_465_382,
    source_type: SourceType::Wikidata,
    name: "UltraEdit Project User Interface",
    extensions: &["pui"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
