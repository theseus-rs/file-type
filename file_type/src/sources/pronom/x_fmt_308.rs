use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_308: FileFormat = FileFormat {
    id: 466,
    puid: "x-fmt/308",
    name: "Btrieve Database",
    extensions: &["btr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
