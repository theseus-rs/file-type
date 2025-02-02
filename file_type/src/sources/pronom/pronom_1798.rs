use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1798: FileFormat = FileFormat {
    id: 1_798,
    source_type: SourceType::Pronom,
    name: "MD5 File",
    extensions: &["md5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
