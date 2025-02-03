use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2150: FileFormat = FileFormat {
    id: 2_150,
    source_type: SourceType::Pronom,
    name: "HP Photo Album",
    extensions: &["albm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
