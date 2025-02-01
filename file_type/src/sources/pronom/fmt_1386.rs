use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1386: FileFormat = FileFormat {
    id: 2_204,
    puid: "fmt/1386",
    name: "Muvee autoProducer Project File",
    extensions: &["mve"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
