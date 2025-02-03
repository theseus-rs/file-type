use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2239: FileFormat = FileFormat {
    id: 2_239,
    source_type: SourceType::Pronom,
    name: "Corel Print House/Print Office Document",
    extensions: &["cph", "cpd", "cpo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
