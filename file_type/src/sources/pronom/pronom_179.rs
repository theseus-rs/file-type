use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_179: FileFormat = FileFormat {
    id: 179,
    source_type: SourceType::Pronom,
    name: "Microsoft Excel Add-In",
    extensions: &["xla", "xll"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
