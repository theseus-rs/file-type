use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_528: FileFormat = FileFormat {
    id: 528,
    source_type: SourceType::Pronom,
    name: "StratGraphics Data File",
    extensions: &["asf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
