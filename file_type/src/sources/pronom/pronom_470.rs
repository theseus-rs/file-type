use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_470: FileFormat = FileFormat {
    id: 470,
    source_type: SourceType::Pronom,
    name: "DesignCAD Drawing",
    extensions: &["dc2", "dc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
