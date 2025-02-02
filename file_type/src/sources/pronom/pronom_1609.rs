use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1609: FileFormat = FileFormat {
    id: 1_609,
    source_type: SourceType::Pronom,
    name: "StarOffice Calc",
    extensions: &["sdc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
