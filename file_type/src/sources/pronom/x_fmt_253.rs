use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_253: FileFormat = FileFormat {
    id: 371,
    puid: "x-fmt/253",
    name: "Microsoft Publisher",
    extensions: &["pub"],
    media_types: &["application/x-mspublisher"],
    internal_signatures: &[],
    related_formats: &[],
};
