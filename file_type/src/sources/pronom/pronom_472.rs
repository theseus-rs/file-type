use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_472: FileFormat = FileFormat {
    id: 472,
    source_type: SourceType::Pronom,
    name: "DesignCAD for Windows Drawing",
    extensions: &["dw2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
