use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_887: FileFormat = FileFormat {
    id: 887,
    source_type: SourceType::Pronom,
    name: "Microsoft PowerPoint for Macintosh",
    extensions: &["ppt"],
    media_types: &["application/vnd.ms-powerpoint"],
    signatures: &[],
    related_formats: &[],
};
