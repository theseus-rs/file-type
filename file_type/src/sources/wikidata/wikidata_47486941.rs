use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47486941: FileFormat = FileFormat {
    id: 47_486_941,
    source_type: SourceType::Wikidata,
    name: "Silo",
    extensions: &["silo"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
