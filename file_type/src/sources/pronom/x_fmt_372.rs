use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_372: FileFormat = FileFormat {
    id: 548,
    puid: "x-fmt/372",
    name: "XYWrite Document",
    extensions: &["xyp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
