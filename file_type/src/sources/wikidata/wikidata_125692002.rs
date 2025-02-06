use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125692002: FileFormat = FileFormat {
    id: 125_692_002,
    source_type: SourceType::Wikidata,
    name: "OpenDocument Graphic Template",
    extensions: &["otg"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
