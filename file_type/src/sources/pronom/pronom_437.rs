use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_437: FileFormat = FileFormat {
    id: 437,
    source_type: SourceType::Pronom,
    name: "DEC Data Exchange File",
    extensions: &["dx"],
    media_types: &["application/dec-dx."],
    internal_signatures: &[],
    related_formats: &[],
};
