use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_470: FileFormat = FileFormat {
    id: 470,
    source_type: SourceType::Pronom,
    name: "DesignCAD Drawing",
    extensions: &["dc2", "dc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
