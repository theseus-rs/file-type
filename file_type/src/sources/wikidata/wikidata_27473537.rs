use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27473537: FileFormat = FileFormat {
    id: 27_473_537,
    source_type: SourceType::Wikidata,
    name: "Advanced Forensic Format, version 3.0",
    extensions: &["aff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
