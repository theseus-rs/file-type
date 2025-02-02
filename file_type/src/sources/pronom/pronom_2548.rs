use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2548: FileFormat = FileFormat {
    id: 2_548,
    source_type: SourceType::Pronom,
    name: "Calc602 Macro File",
    extensions: &["mc6"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
