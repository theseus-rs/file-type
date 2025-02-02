use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1638: FileFormat = FileFormat {
    id: 1_638,
    source_type: SourceType::Pronom,
    name: "Quattro Pro Spreadsheet",
    extensions: &["qpw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSubsequentVersionOf,
        id: 1_637,
    }],
};
