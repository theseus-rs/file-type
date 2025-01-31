use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_293: FileFormat = FileFormat {
    id: 446,
    puid: "x-fmt/293",
    name: "Hewlett Packard Graphics Language",
    extensions: &["hpgl"],
    media_types: &["application/vnd.hp-HPGL"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 1_984,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
