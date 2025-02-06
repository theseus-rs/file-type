use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_524: FileFormat = FileFormat {
    id: 524,
    source_type: SourceType::Pronom,
    name: "Silicon Graphics Graphics File",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
