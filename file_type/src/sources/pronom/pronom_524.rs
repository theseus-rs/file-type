use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_524: FileFormat = FileFormat {
    id: 524,
    source_type: SourceType::Pronom,
    name: "Silicon Graphics Graphics File",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
