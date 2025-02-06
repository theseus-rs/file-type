use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1227: FileFormat = FileFormat {
    id: 1_227,
    source_type: SourceType::Pronom,
    name: "Microsoft Project",
    extensions: &["mpp"],
    media_types: &["application/vnd.ms-project"],
    signatures: &[],
    related_formats: &[],
};
