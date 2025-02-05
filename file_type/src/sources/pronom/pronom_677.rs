use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_677: FileFormat = FileFormat {
    id: 677,
    source_type: SourceType::Pronom,
    name: "Still Picture Interchange File Format",
    extensions: &[],
    media_types: &["image/jpeg"],
    signatures: &[],
    related_formats: &[],
};
