use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_437: FileFormat = FileFormat {
    id: 848,
    puid: "x-fmt/437",
    name: "CATIA Project",
    extensions: &["project"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
