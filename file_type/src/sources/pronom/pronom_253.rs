use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_253: FileFormat = FileFormat {
    id: 253,
    source_type: SourceType::Pronom,
    name: "Instalit Script",
    extensions: &["pvd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
