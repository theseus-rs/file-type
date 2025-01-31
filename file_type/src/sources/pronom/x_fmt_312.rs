use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_312: FileFormat = FileFormat {
    id: 470,
    puid: "x-fmt/312",
    name: "DesignCAD Drawing",
    extensions: &["dc2", "dc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
