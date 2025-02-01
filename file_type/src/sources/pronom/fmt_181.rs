use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_181: FileFormat = FileFormat {
    id: 887,
    puid: "fmt/181",
    name: "Microsoft PowerPoint for Macintosh",
    extensions: &["ppt"],
    media_types: &["application/vnd.ms-powerpoint"],
    internal_signatures: &[],
    related_formats: &[],
};
