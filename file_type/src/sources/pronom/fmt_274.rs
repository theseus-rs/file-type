use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_274: FileFormat = FileFormat {
    id: 1_013,
    puid: "fmt/274",
    name: "SPSS Output File (spv)",
    extensions: &["spv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
