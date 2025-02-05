use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1013: FileFormat = FileFormat {
    id: 1_013,
    source_type: SourceType::Pronom,
    name: "SPSS Output File (spv)",
    extensions: &["spv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
