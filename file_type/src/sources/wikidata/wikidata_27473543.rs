use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27473543: FileFormat = FileFormat {
    id: 27_473_543,
    source_type: SourceType::Wikidata,
    name: "Advanced Forensic Format, version 4",
    extensions: &["aff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
