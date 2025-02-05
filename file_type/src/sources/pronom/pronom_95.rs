use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_95: FileFormat = FileFormat {
    id: 95,
    source_type: SourceType::Pronom,
    name: "Microsoft Excel Web Query",
    extensions: &["iqy"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
