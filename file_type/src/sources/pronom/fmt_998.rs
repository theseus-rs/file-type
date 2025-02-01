use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_998: FileFormat = FileFormat {
    id: 1_803,
    puid: "fmt/998",
    name: "OpenRaster Image Format",
    extensions: &["ora"],
    media_types: &["image/openraster"],
    internal_signatures: &[],
    related_formats: &[],
};
