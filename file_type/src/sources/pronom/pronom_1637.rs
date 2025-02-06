use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1637: FileFormat = FileFormat {
    id: 1_637,
    source_type: SourceType::Pronom,
    name: "Quattro Pro Spreadsheet",
    extensions: &["wb3"],
    media_types: &[],
    signatures: &[],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 1_638,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 1_636,
        },
    ],
};
