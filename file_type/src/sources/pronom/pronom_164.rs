use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_164: FileFormat = FileFormat {
    id: 164,
    source_type: SourceType::Pronom,
    name: "AutoCAD External Database Configuration File",
    extensions: &["udl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
