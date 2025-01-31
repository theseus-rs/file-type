use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_836: FileFormat = FileFormat {
    id: 1_637,
    puid: "fmt/836",
    name: "Quattro Pro Spreadsheet",
    extensions: &["wb3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 1_638,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 1_636,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
