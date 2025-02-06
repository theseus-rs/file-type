use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_522: FileFormat = FileFormat {
    id: 522,
    source_type: SourceType::Pronom,
    name: "SAS for MS-DOS Database",
    extensions: &["ssd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
