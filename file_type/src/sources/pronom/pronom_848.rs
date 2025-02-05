use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_848: FileFormat = FileFormat {
    id: 848,
    source_type: SourceType::Pronom,
    name: "CATIA Project",
    extensions: &["project"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
