use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2259: FileFormat = FileFormat {
    id: 2_259,
    source_type: SourceType::Pronom,
    name: "Apple iWork Document",
    extensions: &["iwa", "key", "pages", "numbers", "template"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
