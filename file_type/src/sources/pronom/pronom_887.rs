use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_887: FileFormat = FileFormat {
    id: 887,
    source_type: SourceType::Pronom,
    name: "Microsoft PowerPoint for Macintosh",
    extensions: &["ppt"],
    media_types: &["application/vnd.ms-powerpoint"],
    internal_signatures: &[],
    related_formats: &[],
};
