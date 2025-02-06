use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2116: FileFormat = FileFormat {
    id: 2_116,
    source_type: SourceType::Pronom,
    name: "Calendar Creator File",
    extensions: &["bcc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSubsequentVersionOf,
        id: 2_115,
    }],
};
