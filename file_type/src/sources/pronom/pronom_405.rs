use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_405: FileFormat = FileFormat {
    id: 405,
    source_type: SourceType::Pronom,
    name: "Microsoft Word for MS-DOS Document",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
