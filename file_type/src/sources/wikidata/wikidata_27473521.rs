use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27473521: FileFormat = FileFormat {
    id: 27_473_521,
    source_type: SourceType::Wikidata,
    name: "Advanced Forensic Format, version 2.0",
    extensions: &["aff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
