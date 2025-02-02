use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1741: FileFormat = FileFormat {
    id: 1_741,
    source_type: SourceType::Pronom,
    name: "Microsoft Picture It! Image File",
    extensions: &["mix"],
    media_types: &["image/vnd.mix"],
    internal_signatures: &[],
    related_formats: &[],
};
