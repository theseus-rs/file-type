use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130280361: FileFormat = FileFormat {
    id: 130_280_361,
    source_type: SourceType::Wikidata,
    name: "Mason file format",
    extensions: &["m"],
    media_types: &["application/x-mason"],
    signatures: &[],
    related_formats: &[],
};
