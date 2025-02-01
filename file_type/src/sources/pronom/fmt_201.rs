use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_201: FileFormat = FileFormat {
    id: 926,
    puid: "fmt/201",
    name: "Mathematica Notebook",
    extensions: &["nb"],
    media_types: &["application/mathematica"],
    internal_signatures: &[],
    related_formats: &[],
};
