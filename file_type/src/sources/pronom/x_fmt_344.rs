use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_344: FileFormat = FileFormat {
    id: 508,
    puid: "x-fmt/344",
    name: "Microsoft Works Database",
    extensions: &["bdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
