use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110443436: FileFormat = FileFormat {
    id: 110_443_436,
    source_type: SourceType::Wikidata,
    name: "Bentley Microstation Hidden Line File",
    extensions: &["hln"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
