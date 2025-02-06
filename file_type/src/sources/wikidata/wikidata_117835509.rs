use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117835509: FileFormat = FileFormat {
    id: 117_835_509,
    source_type: SourceType::Wikidata,
    name: "Generic fax format",
    extensions: &["cg3"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
