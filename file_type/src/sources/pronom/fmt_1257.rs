use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1257: FileFormat = FileFormat {
    id: 2_075,
    puid: "fmt/1257",
    name: "AutoCAD Temporary File",
    extensions: &["ac$"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
