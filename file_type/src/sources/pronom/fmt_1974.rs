use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1974: FileFormat = FileFormat {
    id: 2_841,
    puid: "fmt/1974",
    name: "Finale Notation File",
    extensions: &["musx"],
    media_types: &["application/vnd.makemusic.notation"],
    internal_signatures: &[],
    related_formats: &[],
};
