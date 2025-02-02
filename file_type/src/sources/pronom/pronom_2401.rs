use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2401: FileFormat = FileFormat {
    id: 2_401,
    source_type: SourceType::Pronom,
    name: "Spectrum 512 Uncompressed | Spectrum 512 Uncompressed Enhanced",
    extensions: &["spu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 687,
    }],
};
