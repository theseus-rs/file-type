use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_71: FileFormat = FileFormat {
    id: 112,
    puid: "x-fmt/71",
    name: "AutoCAD Source Menu File",
    extensions: &["mns"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
