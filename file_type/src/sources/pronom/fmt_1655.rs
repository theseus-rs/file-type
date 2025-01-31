use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1655: FileFormat = FileFormat {
    id: 2_482,
    puid: "fmt/1655",
    name: "cdrLabel Label File",
    extensions: &["clb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
