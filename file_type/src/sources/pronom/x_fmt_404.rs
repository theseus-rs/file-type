use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_404: FileFormat = FileFormat {
    id: 758,
    puid: "x-fmt/404",
    name: "StarOffice Calc",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
