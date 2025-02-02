use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_213: FileFormat = FileFormat {
    id: 213,
    source_type: SourceType::Pronom,
    name: "Micrografx Designer",
    extensions: &["dsf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
