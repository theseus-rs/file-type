use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_62445151: FileFormat = FileFormat {
    id: 62_445_151,
    source_type: SourceType::Wikidata,
    name: "OWL Functional-Style Syntax",
    extensions: &["ofn"],
    media_types: &["text/owl-functional"],
    signatures: &[],
    related_formats: &[],
};
