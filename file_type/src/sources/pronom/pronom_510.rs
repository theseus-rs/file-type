use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_510: FileFormat = FileFormat {
    id: 510,
    source_type: SourceType::Pronom,
    name: "Microstation CAD Drawing",
    extensions: &["dgn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
