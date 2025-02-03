use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_466: FileFormat = FileFormat {
    id: 466,
    source_type: SourceType::Pronom,
    name: "Btrieve Database",
    extensions: &["btr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
