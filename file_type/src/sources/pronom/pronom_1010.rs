use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1010: FileFormat = FileFormat {
    id: 1_010,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Word Processor Macintosh",
    extensions: &["wps"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
