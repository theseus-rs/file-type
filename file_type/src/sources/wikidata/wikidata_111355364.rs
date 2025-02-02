use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111355364: FileFormat = FileFormat {
    id: 111_355_364,
    source_type: SourceType::Wikidata,
    name: "Covox 8-bit audio",
    extensions: &["v8"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
