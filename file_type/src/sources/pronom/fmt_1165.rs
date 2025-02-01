use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1165: FileFormat = FileFormat {
    id: 1_975,
    puid: "fmt/1165",
    name: "Praat Script File",
    extensions: &["praat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
