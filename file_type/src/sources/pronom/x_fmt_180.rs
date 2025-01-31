use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_180: FileFormat = FileFormat {
    id: 253,
    puid: "x-fmt/180",
    name: "Instalit Script",
    extensions: &["pvd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
