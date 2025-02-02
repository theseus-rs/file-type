use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1706: FileFormat = FileFormat {
    id: 1_706,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Spreadsheet",
    extensions: &["xlr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 684,
    }],
};
