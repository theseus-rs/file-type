use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2475: FileFormat = FileFormat {
    id: 2_475,
    source_type: SourceType::Pronom,
    name: "Crystal Reports File",
    extensions: &["rpt"],
    media_types: &["application/x-rpt"],
    internal_signatures: &[],
    related_formats: &[],
};
