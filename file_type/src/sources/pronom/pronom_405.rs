use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_405: FileFormat = FileFormat {
    id: 405,
    source_type: SourceType::Pronom,
    name: "Microsoft Word for MS-DOS Document",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
