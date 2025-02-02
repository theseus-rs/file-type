use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_508: FileFormat = FileFormat {
    id: 508,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Database",
    extensions: &["bdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
