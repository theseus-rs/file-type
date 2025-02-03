use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110443436: FileFormat = FileFormat {
    id: 110_443_436,
    source_type: SourceType::Wikidata,
    name: "Bentley Microstation Hidden Line File",
    extensions: &["hln"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
