use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1523: FileFormat = FileFormat {
    id: 1_523,
    source_type: SourceType::Pronom,
    name: "Keyhole Markup Language (Container)",
    extensions: &["kmz"],
    media_types: &["application/vnd.google-earth.kmz"],
    signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::EquivalentTo,
        id: 975,
    }],
};
