use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_148: FileFormat = FileFormat {
    id: 148,
    source_type: SourceType::Pronom,
    name: "AutoCAD Script",
    extensions: &["scr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
