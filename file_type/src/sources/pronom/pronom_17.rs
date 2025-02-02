use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_17: FileFormat = FileFormat {
    id: 17,
    source_type: SourceType::Pronom,
    name: "FoxPro Database",
    extensions: &["dbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
