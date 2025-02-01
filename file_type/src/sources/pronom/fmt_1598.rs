use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1598: FileFormat = FileFormat {
    id: 2_425,
    puid: "fmt/1598",
    name: "Stata .do Command File",
    extensions: &["do"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
