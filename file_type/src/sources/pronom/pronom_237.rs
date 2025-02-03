use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_237: FileFormat = FileFormat {
    id: 237,
    source_type: SourceType::Pronom,
    name: "Kodak PhotoCD Image",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
