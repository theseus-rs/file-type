use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_468: FileFormat = FileFormat {
    id: 468,
    source_type: SourceType::Pronom,
    name: "CorelCHART Document",
    extensions: &["cch"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
