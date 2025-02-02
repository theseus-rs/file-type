use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1523: FileFormat = FileFormat {
    id: 1_523,
    source_type: SourceType::Pronom,
    name: "Keyhole Markup Language (Container)",
    extensions: &["kmz"],
    media_types: &["application/vnd.google-earth.kmz"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::EquivalentTo,
        id: 975,
    }],
};
