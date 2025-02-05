use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2111: FileFormat = FileFormat {
    id: 2_111,
    source_type: SourceType::Pronom,
    name: "602Text Document",
    extensions: &["wpd", "wpt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
