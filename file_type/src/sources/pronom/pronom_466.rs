use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_466: FileFormat = FileFormat {
    id: 466,
    source_type: SourceType::Pronom,
    name: "Btrieve Database",
    extensions: &["btr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
