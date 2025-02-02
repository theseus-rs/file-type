use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1203: FileFormat = FileFormat {
    id: 1_203,
    source_type: SourceType::Pronom,
    name: "Adobe Illustrator",
    extensions: &["ai"],
    media_types: &["application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};
