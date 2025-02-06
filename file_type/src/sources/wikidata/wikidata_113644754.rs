use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113644754: FileFormat = FileFormat {
    id: 113_644_754,
    source_type: SourceType::Wikidata,
    name: "Hayes JT FAX",
    extensions: &["001"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
