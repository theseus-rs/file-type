use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_237: FileFormat = FileFormat {
    id: 237,
    source_type: SourceType::Pronom,
    name: "Kodak PhotoCD Image",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
