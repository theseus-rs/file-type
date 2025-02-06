use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123436243: FileFormat = FileFormat {
    id: 123_436_243,
    source_type: SourceType::Wikidata,
    name: "CD Style Sheet file",
    extensions: &["cds"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
