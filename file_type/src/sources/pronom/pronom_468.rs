use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_468: FileFormat = FileFormat {
    id: 468,
    source_type: SourceType::Pronom,
    name: "CorelCHART Document",
    extensions: &["cch"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
