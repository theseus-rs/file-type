use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28756571: FileFormat = FileFormat {
    id: 28_756_571,
    source_type: SourceType::Wikidata,
    name: "Forth script",
    extensions: &["fth"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
