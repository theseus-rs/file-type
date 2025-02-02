use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_510: FileFormat = FileFormat {
    id: 510,
    source_type: SourceType::Pronom,
    name: "Microstation CAD Drawing",
    extensions: &["dgn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
