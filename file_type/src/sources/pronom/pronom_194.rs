use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_194: FileFormat = FileFormat {
    id: 194,
    source_type: SourceType::Pronom,
    name: "Audio Interchange File Format",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
