use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_62128473: FileFormat = FileFormat {
    id: 62_128_473,
    source_type: SourceType::Wikidata,
    name: "CSV Schema",
    extensions: &["csvs"],
    media_types: &["text/csv-schema"],
    internal_signatures: &[],
    related_formats: &[],
};
