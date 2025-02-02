use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125692002: FileFormat = FileFormat {
    id: 125_692_002,
    source_type: SourceType::Wikidata,
    name: "OpenDocument Graphic Template",
    extensions: &["otg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
