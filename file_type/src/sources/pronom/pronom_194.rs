use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_194: FileFormat = FileFormat {
    id: 194,
    source_type: SourceType::Pronom,
    name: "Audio Interchange File Format",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
