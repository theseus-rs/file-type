use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_102: FileFormat = FileFormat {
    id: 102,
    source_type: SourceType::Pronom,
    name: "AutoCAD Landscape Library",
    extensions: &["lli"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
