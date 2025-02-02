use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_85: FileFormat = FileFormat {
    id: 85,
    source_type: SourceType::Pronom,
    name: "Drawing Interchange Format Style Extract",
    extensions: &["dxx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
