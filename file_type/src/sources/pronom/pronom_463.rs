use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_463: FileFormat = FileFormat {
    id: 463,
    source_type: SourceType::Pronom,
    name: "Apple Sound",
    extensions: &["afc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
