use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1011: FileFormat = FileFormat {
    id: 1_011,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Word Processor Macintosh",
    extensions: &["wps"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
