use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_102: FileFormat = FileFormat {
    id: 102,
    source_type: SourceType::Pronom,
    name: "AutoCAD Landscape Library",
    extensions: &["lli"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
