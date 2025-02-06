use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1706: FileFormat = FileFormat {
    id: 1_706,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Spreadsheet",
    extensions: &["xlr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 684,
    }],
};
