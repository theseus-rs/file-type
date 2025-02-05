use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_268: FileFormat = FileFormat {
    id: 268,
    source_type: SourceType::Pronom,
    name: "Standard Generalized Markup Language",
    extensions: &["sgml", "sgm"],
    media_types: &["text/sgml"],
    signatures: &[],
    related_formats: &[],
};
