use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_273: FileFormat = FileFormat {
    id: 1_011,
    puid: "fmt/273",
    name: "Microsoft Works Word Processor Macintosh",
    extensions: &["wps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
