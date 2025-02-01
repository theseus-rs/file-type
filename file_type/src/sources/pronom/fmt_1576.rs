use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1576: FileFormat = FileFormat {
    id: 2_401,
    puid: "fmt/1576",
    name: "Spectrum 512 Uncompressed | Spectrum 512 Uncompressed Enhanced",
    extensions: &["spu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 687,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
