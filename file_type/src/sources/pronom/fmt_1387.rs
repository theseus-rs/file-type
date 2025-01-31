use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1387: FileFormat = FileFormat {
    id: 2_205,
    puid: "fmt/1387",
    name: "Muvee autoProducer Project File",
    extensions: &["mvex"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
