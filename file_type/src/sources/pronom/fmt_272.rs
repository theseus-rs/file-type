use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_272: FileFormat = FileFormat {
    id: 1_010,
    puid: "fmt/272",
    name: "Microsoft Works Word Processor Macintosh",
    extensions: &["wps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
