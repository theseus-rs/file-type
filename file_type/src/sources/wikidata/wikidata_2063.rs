use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_2063: FileFormat = FileFormat {
    id: 2_063,
    source_type: SourceType::Wikidata,
    name: "JSON",
    extensions: &["json"],
    media_types: &["application/json"],
    signatures: &[],
    related_formats: &[],
};
