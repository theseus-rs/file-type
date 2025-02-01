use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_25: FileFormat = FileFormat {
    id: 55,
    puid: "x-fmt/25",
    name: "OS/2 Bitmap",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 728,
        relationship_type: RelationshipType::EquivalentTo,
    }],
};
