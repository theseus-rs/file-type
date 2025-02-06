use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_42: FileFormat = FileFormat {
    id: 42,
    source_type: SourceType::Pronom,
    name: "MS-DOS Text File",
    extensions: &[],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
