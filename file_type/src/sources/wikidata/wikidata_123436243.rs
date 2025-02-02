use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123436243: FileFormat = FileFormat {
    id: 123_436_243,
    source_type: SourceType::Wikidata,
    name: "CD Style Sheet file",
    extensions: &["cds"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
