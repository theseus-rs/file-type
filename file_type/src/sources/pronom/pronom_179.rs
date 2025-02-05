use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_179: FileFormat = FileFormat {
    id: 179,
    source_type: SourceType::Pronom,
    name: "Microsoft Excel Add-In",
    extensions: &["xla", "xll"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
