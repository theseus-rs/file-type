use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1013: FileFormat = FileFormat {
    id: 1_013,
    source_type: SourceType::Pronom,
    name: "SPSS Output File (spv)",
    extensions: &["spv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
