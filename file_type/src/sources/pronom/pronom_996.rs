use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_996: FileFormat = FileFormat {
    id: 996,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Word Processor 5-6",
    extensions: &["wps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
