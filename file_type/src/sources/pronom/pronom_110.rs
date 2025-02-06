use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_110: FileFormat = FileFormat {
    id: 110,
    source_type: SourceType::Pronom,
    name: "AutoLISP Menu Source File",
    extensions: &["mnl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
