use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_95: FileFormat = FileFormat {
    id: 95,
    source_type: SourceType::Pronom,
    name: "Microsoft Excel Web Query",
    extensions: &["iqy"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
