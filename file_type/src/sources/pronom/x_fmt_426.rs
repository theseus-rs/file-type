use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_426: FileFormat = FileFormat {
    id: 813,
    puid: "x-fmt/426",
    name: "License file",
    extensions: &["lic"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
