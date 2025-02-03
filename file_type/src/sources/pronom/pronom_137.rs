use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_137: FileFormat = FileFormat {
    id: 137,
    source_type: SourceType::Pronom,
    name: "Microsoft Print File",
    extensions: &["prn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
