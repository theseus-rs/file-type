use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_438: FileFormat = FileFormat {
    id: 438,
    source_type: SourceType::Pronom,
    name: "DEC WPS Plus Document",
    extensions: &["wpl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
