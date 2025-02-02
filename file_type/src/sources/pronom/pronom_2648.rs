use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2648: FileFormat = FileFormat {
    id: 2_648,
    source_type: SourceType::Pronom,
    name: "SHA512 File",
    extensions: &["sha512"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
