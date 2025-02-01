use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_403: FileFormat = FileFormat {
    id: 757,
    puid: "x-fmt/403",
    name: "StarOffice Writer",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
