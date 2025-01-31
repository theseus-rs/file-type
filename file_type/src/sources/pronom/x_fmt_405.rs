use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_405: FileFormat = FileFormat {
    id: 759,
    puid: "x-fmt/405",
    name: "StarOffice Impress",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
