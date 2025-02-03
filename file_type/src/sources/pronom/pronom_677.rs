use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_677: FileFormat = FileFormat {
    id: 677,
    source_type: SourceType::Pronom,
    name: "Still Picture Interchange File Format",
    extensions: &[],
    media_types: &["image/jpeg"],
    internal_signatures: &[],
    related_formats: &[],
};
