use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2666: FileFormat = FileFormat {
    id: 2_666,
    source_type: SourceType::Pronom,
    name: "Adobe Color Swatch",
    extensions: &["aco"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
