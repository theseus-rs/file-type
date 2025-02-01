use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1678: FileFormat = FileFormat {
    id: 2_514,
    puid: "fmt/1678",
    name: "MATLAB Script File",
    extensions: &["m"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
