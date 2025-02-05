use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_527: FileFormat = FileFormat {
    id: 527,
    source_type: SourceType::Pronom,
    name: "StatGraphics Data File",
    extensions: &["aws"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
