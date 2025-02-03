use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2235: FileFormat = FileFormat {
    id: 2_235,
    source_type: SourceType::Pronom,
    name: "Corel Print House Document",
    extensions: &["cph", "cpd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
