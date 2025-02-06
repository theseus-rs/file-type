use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_55: FileFormat = FileFormat {
    id: 55,
    source_type: SourceType::Pronom,
    name: "OS/2 Bitmap",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::EquivalentTo,
        id: 728,
    }],
};
