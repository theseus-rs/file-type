use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_164: FileFormat = FileFormat {
    id: 164,
    source_type: SourceType::Pronom,
    name: "AutoCAD External Database Configuration File",
    extensions: &["udl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
