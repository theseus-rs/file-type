use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_253: FileFormat = FileFormat {
    id: 253,
    source_type: SourceType::Pronom,
    name: "Instalit Script",
    extensions: &["pvd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
