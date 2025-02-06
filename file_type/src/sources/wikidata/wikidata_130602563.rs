use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130602563: FileFormat = FileFormat {
    id: 130_602_563,
    source_type: SourceType::Wikidata,
    name: "ReasonML file format",
    extensions: &["re", "rei"],
    media_types: &["text/x-reasonml"],
    signatures: &[],
    related_formats: &[],
};
