use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_58006953: FileFormat = FileFormat {
    id: 58_006_953,
    source_type: SourceType::Wikidata,
    name: "TRIM Context Reference File",
    extensions: &["tr5", "txt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
