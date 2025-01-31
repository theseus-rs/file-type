use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_69: FileFormat = FileFormat {
    id: 110,
    puid: "x-fmt/69",
    name: "AutoLISP Menu Source File",
    extensions: &["mnl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
