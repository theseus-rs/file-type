use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_990: FileFormat = FileFormat {
    id: 1_795,
    puid: "fmt/990",
    name: "ESRI File Geodatabase",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
