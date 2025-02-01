use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1423: FileFormat = FileFormat {
    id: 2_241,
    puid: "fmt/1423",
    name: "HP TRIM Outlook Saved Message File",
    extensions: &["vmbx", "mbx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
