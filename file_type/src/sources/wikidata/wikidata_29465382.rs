use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29465382: FileFormat = FileFormat {
    id: 29_465_382,
    source_type: SourceType::Wikidata,
    name: "UltraEdit Project User Interface",
    extensions: &["pui"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
