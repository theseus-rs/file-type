use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_268: FileFormat = FileFormat {
    id: 1_006,
    puid: "fmt/268",
    name: "Microsoft Works Database for Macintosh",
    extensions: &["wdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
