use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_218: FileFormat = FileFormat {
    id: 946,
    puid: "fmt/218",
    name: "Microsoft FrontPage",
    extensions: &["lck"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
