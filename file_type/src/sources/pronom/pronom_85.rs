use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_85: FileFormat = FileFormat {
    id: 85,
    source_type: SourceType::Pronom,
    name: "Drawing Interchange Format Style Extract",
    extensions: &["dxx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
