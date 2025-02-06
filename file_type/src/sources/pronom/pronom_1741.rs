use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1741: FileFormat = FileFormat {
    id: 1_741,
    source_type: SourceType::Pronom,
    name: "Microsoft Picture It! Image File",
    extensions: &["mix"],
    media_types: &["image/vnd.mix"],
    signatures: &[],
    related_formats: &[],
};
