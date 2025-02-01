use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_200: FileFormat = FileFormat {
    id: 274,
    puid: "x-fmt/200",
    name: "PageMaker Time Stamp File",
    extensions: &["tym"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
