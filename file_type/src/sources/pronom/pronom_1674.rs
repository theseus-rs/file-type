use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1674: FileFormat = FileFormat {
    id: 1_674,
    source_type: SourceType::Pronom,
    name: "Perl Script",
    extensions: &["pl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
