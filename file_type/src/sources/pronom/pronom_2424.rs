use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2424: FileFormat = FileFormat {
    id: 2_424,
    source_type: SourceType::Pronom,
    name: "PageMaker Template File",
    extensions: &["pt5"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
