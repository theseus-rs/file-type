use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_265: FileFormat = FileFormat {
    id: 1_003,
    puid: "fmt/265",
    name: "Microsoft Works Word Processor DOS",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
