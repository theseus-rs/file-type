use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_58006953: FileFormat = FileFormat {
    id: 58_006_953,
    source_type: SourceType::Wikidata,
    name: "TRIM Context Reference File",
    extensions: &["tr5", "txt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
