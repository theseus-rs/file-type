use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_13: FileFormat = FileFormat {
    id: 13,
    source_type: SourceType::Pronom,
    name: "Online Description Tool Format",
    extensions: &["odt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
