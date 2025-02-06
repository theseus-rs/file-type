use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1203: FileFormat = FileFormat {
    id: 1_203,
    source_type: SourceType::Pronom,
    name: "Adobe Illustrator",
    extensions: &["ai"],
    media_types: &["application/postscript"],
    signatures: &[],
    related_formats: &[],
};
