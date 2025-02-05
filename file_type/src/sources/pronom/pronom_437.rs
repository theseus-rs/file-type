use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_437: FileFormat = FileFormat {
    id: 437,
    source_type: SourceType::Pronom,
    name: "DEC Data Exchange File",
    extensions: &["dx"],
    media_types: &["application/dec-dx."],
    signatures: &[],
    related_formats: &[],
};
