use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_178: FileFormat = FileFormat {
    id: 884,
    puid: "fmt/178",
    name: "Microsoft Excel for Macintosh",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
