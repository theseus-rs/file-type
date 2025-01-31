use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_258: FileFormat = FileFormat {
    id: 996,
    puid: "fmt/258",
    name: "Microsoft Works Word Processor 5-6",
    extensions: &["wps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
