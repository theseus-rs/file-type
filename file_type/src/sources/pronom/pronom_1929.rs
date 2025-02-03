use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1929: FileFormat = FileFormat {
    id: 1_929,
    source_type: SourceType::Pronom,
    name: "Jupyter Python Notebook",
    extensions: &["ipynb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
