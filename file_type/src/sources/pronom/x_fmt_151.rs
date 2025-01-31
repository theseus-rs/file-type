use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_151: FileFormat = FileFormat {
    id: 213,
    puid: "x-fmt/151",
    name: "Micrografx Designer",
    extensions: &["dsf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
