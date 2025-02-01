use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_333: FileFormat = FileFormat {
    id: 496,
    puid: "x-fmt/333",
    name: "Lotus Approach View File",
    extensions: &["apr"],
    media_types: &["application/vnd.lotus-approach"],
    internal_signatures: &[],
    related_formats: &[],
};
