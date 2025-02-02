use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_888: FileFormat = FileFormat {
    id: 888,
    source_type: SourceType::Pronom,
    name: "Microsoft PowerPoint for Macintosh",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
