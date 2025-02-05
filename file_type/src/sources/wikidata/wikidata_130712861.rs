use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130712861: FileFormat = FileFormat {
    id: 130_712_861,
    source_type: SourceType::Wikidata,
    name: "Relation Query Language file format",
    extensions: &["rql"],
    media_types: &["text/x-rql"],
    signatures: &[],
    related_formats: &[],
};
