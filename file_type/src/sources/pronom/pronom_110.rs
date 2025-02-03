use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_110: FileFormat = FileFormat {
    id: 110,
    source_type: SourceType::Pronom,
    name: "AutoLISP Menu Source File",
    extensions: &["mnl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
