use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_125: FileFormat = FileFormat {
    id: 125,
    source_type: SourceType::Pronom,
    name: "Inkwriter/Notetaker Template",
    extensions: &["pdt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
