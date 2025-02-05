use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29650547: FileFormat = FileFormat {
    id: 29_650_547,
    source_type: SourceType::Wikidata,
    name: "Package",
    extensions: &["pack"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
