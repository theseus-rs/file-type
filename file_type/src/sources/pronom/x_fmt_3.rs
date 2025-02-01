use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_3: FileFormat = FileFormat {
    id: 13,
    puid: "x-fmt/3",
    name: "Online Description Tool Format",
    extensions: &["odt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
