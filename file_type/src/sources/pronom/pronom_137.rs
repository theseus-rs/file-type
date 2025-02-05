use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_137: FileFormat = FileFormat {
    id: 137,
    source_type: SourceType::Pronom,
    name: "Microsoft Print File",
    extensions: &["prn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
