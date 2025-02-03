use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_189: FileFormat = FileFormat {
    id: 189,
    source_type: SourceType::Pronom,
    name: "MS-DOS Text File with line breaks",
    extensions: &[],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
