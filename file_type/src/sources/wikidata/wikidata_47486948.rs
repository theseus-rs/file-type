use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47486948: FileFormat = FileFormat {
    id: 47_486_948,
    source_type: SourceType::Wikidata,
    name: "Silo",
    extensions: &["silo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
