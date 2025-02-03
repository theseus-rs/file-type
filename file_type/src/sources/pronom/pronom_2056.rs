use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2056: FileFormat = FileFormat {
    id: 2_056,
    source_type: SourceType::Pronom,
    name: "Band Interleaved By Line (BIL) Image Encoding",
    extensions: &["bil"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
