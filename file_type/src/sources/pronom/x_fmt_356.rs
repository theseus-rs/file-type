use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_356: FileFormat = FileFormat {
    id: 522,
    puid: "x-fmt/356",
    name: "SAS for MS-DOS Database",
    extensions: &["ssd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
