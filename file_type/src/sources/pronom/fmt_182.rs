use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_182: FileFormat = FileFormat {
    id: 888,
    puid: "fmt/182",
    name: "Microsoft PowerPoint for Macintosh",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
