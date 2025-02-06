use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1743: FileFormat = FileFormat {
    id: 1_743,
    source_type: SourceType::Pronom,
    name: "Python Source Code File",
    extensions: &["py"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
