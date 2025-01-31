use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_125: FileFormat = FileFormat {
    id: 180,
    puid: "x-fmt/125",
    name: "Microsoft Excel Toolbar",
    extensions: &["xlb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
