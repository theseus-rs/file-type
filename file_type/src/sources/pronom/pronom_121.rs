use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_121: FileFormat = FileFormat {
    id: 121,
    source_type: SourceType::Pronom,
    name: "AutoCAD Plot Configuration File",
    extensions: &["pcp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
