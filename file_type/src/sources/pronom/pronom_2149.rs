use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2149: FileFormat = FileFormat {
    id: 2_149,
    source_type: SourceType::Pronom,
    name: "Avery DesignPro Document",
    extensions: &["zdl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
