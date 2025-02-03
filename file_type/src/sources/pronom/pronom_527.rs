use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_527: FileFormat = FileFormat {
    id: 527,
    source_type: SourceType::Pronom,
    name: "StatGraphics Data File",
    extensions: &["aws"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
