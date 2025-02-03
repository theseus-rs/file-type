use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_758: FileFormat = FileFormat {
    id: 758,
    source_type: SourceType::Pronom,
    name: "StarOffice Calc",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
