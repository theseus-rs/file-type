use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_528: FileFormat = FileFormat {
    id: 528,
    source_type: SourceType::Pronom,
    name: "StratGraphics Data File",
    extensions: &["asf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
