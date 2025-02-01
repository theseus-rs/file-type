use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_266: FileFormat = FileFormat {
    id: 1_004,
    puid: "fmt/266",
    name: "Microsoft Works Word Processor DOS",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
