use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2627: FileFormat = FileFormat {
    id: 2_627,
    source_type: SourceType::Pronom,
    name: "SIARD (Software-Independent Archiving of Relational Databases)",
    extensions: &["siard"],
    media_types: &[],
    signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSubsequentVersionOf,
        id: 2_006,
    }],
};
