use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2149: FileFormat = FileFormat {
    id: 2_149,
    source_type: SourceType::Pronom,
    name: "Avery DesignPro Document",
    extensions: &["zdl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
