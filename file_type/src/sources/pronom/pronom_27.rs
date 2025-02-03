use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_27: FileFormat = FileFormat {
    id: 27,
    source_type: SourceType::Pronom,
    name: "Revisable-Form-Text Document Content Architecture",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
