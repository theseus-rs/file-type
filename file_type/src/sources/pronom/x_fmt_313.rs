use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_313: FileFormat = FileFormat {
    id: 472,
    puid: "x-fmt/313",
    name: "DesignCAD for Windows Drawing",
    extensions: &["dw2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
