use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27473543: FileFormat = FileFormat {
    id: 27_473_543,
    source_type: SourceType::Wikidata,
    name: "Advanced Forensic Format, version 4",
    extensions: &["aff"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
