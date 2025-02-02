use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_55: FileFormat = FileFormat {
    id: 55,
    source_type: SourceType::Pronom,
    name: "OS/2 Bitmap",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::EquivalentTo,
        id: 728,
    }],
};
