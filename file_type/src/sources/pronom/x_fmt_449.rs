use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_449: FileFormat = FileFormat {
    id: 864,
    puid: "x-fmt/449",
    name: "Steel Detailing Neutral Format",
    extensions: &["sdn"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
