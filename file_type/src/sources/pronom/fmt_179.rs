use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_179: FileFormat = FileFormat {
    id: 885,
    puid: "fmt/179",
    name: "Microsoft PowerPoint for Macintosh",
    extensions: &["ppt"],
    media_types: &["application/vnd.ms-powerpoint"],
    internal_signatures: &[],
    related_formats: &[],
};
