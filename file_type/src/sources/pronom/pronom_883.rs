use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_883: FileFormat = FileFormat {
    id: 883,
    source_type: SourceType::Pronom,
    name: "Microsoft Excel for Macintosh",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
