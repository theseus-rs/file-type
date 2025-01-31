use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_152: FileFormat = FileFormat {
    id: 214,
    puid: "x-fmt/152",
    name: "Digital Video",
    extensions: &["dv"],
    media_types: &["video/dv"],
    internal_signatures: &[],
    related_formats: &[],
};
