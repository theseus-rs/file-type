use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_888: FileFormat = FileFormat {
    id: 888,
    source_type: SourceType::Pronom,
    name: "Microsoft PowerPoint for Macintosh",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
