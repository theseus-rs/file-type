use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1213: FileFormat = FileFormat {
    id: 2_023,
    puid: "fmt/1213",
    name: "Zoner Callisto Metafile",
    extensions: &["zmf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
