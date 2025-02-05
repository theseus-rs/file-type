use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_13: FileFormat = FileFormat {
    id: 13,
    source_type: SourceType::Pronom,
    name: "Online Description Tool Format",
    extensions: &["odt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
