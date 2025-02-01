use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1340: FileFormat = FileFormat {
    id: 2_158,
    puid: "fmt/1340",
    name: "BDOC",
    extensions: &["bdoc"],
    media_types: &["application/vnd.bdoc-1.0"],
    internal_signatures: &[],
    related_formats: &[],
};
