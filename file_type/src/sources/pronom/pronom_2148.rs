use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2148: FileFormat = FileFormat {
    id: 2_148,
    source_type: SourceType::Pronom,
    name: "Avery DesignPro Document",
    extensions: &["zdp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
