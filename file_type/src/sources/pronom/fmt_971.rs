use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_971: FileFormat = FileFormat {
    id: 1_776,
    puid: "fmt/971",
    name: "Microsoft Windows Movie Maker File",
    extensions: &["mswmm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
