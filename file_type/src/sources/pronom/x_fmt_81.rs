use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_81: FileFormat = FileFormat {
    id: 125,
    puid: "x-fmt/81",
    name: "Inkwriter/Notetaker Template",
    extensions: &["pdt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
