use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1798: FileFormat = FileFormat {
    id: 1_798,
    source_type: SourceType::Pronom,
    name: "MD5 File",
    extensions: &["md5"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
