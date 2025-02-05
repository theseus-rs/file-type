use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_16251944: FileFormat = FileFormat {
    id: 16_251_944,
    source_type: SourceType::Wikidata,
    name: "TransXChange",
    extensions: &["txc", "xml"],
    media_types: &["application/xml"],
    signatures: &[],
    related_formats: &[],
};
