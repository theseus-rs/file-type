use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_148: FileFormat = FileFormat {
    id: 148,
    source_type: SourceType::Pronom,
    name: "AutoCAD Script",
    extensions: &["scr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
