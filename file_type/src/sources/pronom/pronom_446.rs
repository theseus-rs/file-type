use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_446: FileFormat = FileFormat {
    id: 446,
    source_type: SourceType::Pronom,
    name: "Hewlett Packard Graphics Language",
    extensions: &["hpgl"],
    media_types: &["application/vnd.hp-HPGL"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsPreviousVersionOf,
        id: 1_984,
    }],
};
