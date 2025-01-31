use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1852: FileFormat = FileFormat {
    id: 2_705,
    puid: "fmt/1852",
    name: "Camtasia Recording File",
    extensions: &["camrec"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
