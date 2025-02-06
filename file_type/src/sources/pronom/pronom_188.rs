use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_188: FileFormat = FileFormat {
    id: 188,
    source_type: SourceType::Pronom,
    name: "Microsoft Word for Macintosh Document",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
