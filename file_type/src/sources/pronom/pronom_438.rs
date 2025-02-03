use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_438: FileFormat = FileFormat {
    id: 438,
    source_type: SourceType::Pronom,
    name: "DEC WPS Plus Document",
    extensions: &["wpl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
