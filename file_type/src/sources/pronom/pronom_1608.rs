use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1608: FileFormat = FileFormat {
    id: 1_608,
    source_type: SourceType::Pronom,
    name: "StarOffice Calc",
    extensions: &["sdc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
