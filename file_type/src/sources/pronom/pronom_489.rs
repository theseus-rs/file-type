use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_489: FileFormat = FileFormat {
    id: 489,
    source_type: SourceType::Pronom,
    name: "Hewlett Packard AdvanceWrite Text File",
    extensions: &["aw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
