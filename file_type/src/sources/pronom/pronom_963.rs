use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_963: FileFormat = FileFormat {
    id: 963,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Word Processor 3-4 for Windows",
    extensions: &["wps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
