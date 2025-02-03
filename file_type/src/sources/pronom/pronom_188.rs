use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_188: FileFormat = FileFormat {
    id: 188,
    source_type: SourceType::Pronom,
    name: "Microsoft Word for Macintosh Document",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
