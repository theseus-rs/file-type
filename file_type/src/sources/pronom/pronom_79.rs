use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_79: FileFormat = FileFormat {
    id: 79,
    source_type: SourceType::Pronom,
    name: "Visual Basic Macro",
    extensions: &["dvb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
