use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_233: FileFormat = FileFormat {
    id: 963,
    puid: "fmt/233",
    name: "Microsoft Works Word Processor 3-4 for Windows",
    extensions: &["wps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
