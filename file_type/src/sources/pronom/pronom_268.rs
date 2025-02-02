use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_268: FileFormat = FileFormat {
    id: 268,
    source_type: SourceType::Pronom,
    name: "Standard Generalized Markup Language",
    extensions: &["sgml", "sgm"],
    media_types: &["text/sgml"],
    internal_signatures: &[],
    related_formats: &[],
};
