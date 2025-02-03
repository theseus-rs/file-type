use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_121: FileFormat = FileFormat {
    id: 121,
    source_type: SourceType::Pronom,
    name: "AutoCAD Plot Configuration File",
    extensions: &["pcp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
