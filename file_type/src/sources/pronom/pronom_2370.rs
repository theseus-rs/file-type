use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2370: FileFormat = FileFormat {
    id: 2_370,
    source_type: SourceType::Pronom,
    name: "NeoDesk Icon File",
    extensions: &["nic"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
