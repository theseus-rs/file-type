use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_61: FileFormat = FileFormat {
    id: 61,
    source_type: SourceType::Pronom,
    name: "CorelDraw Template",
    extensions: &["cdt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
