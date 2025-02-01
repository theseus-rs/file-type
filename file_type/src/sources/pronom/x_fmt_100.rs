use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_100: FileFormat = FileFormat {
    id: 148,
    puid: "x-fmt/100",
    name: "AutoCAD Script",
    extensions: &["scr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
