use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_23: FileFormat = FileFormat {
    id: 52,
    puid: "x-fmt/23",
    name: "Microsoft Excel Backup",
    extensions: &["xlk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
