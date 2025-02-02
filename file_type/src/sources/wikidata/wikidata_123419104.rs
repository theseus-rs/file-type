use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123419104: FileFormat = FileFormat {
    id: 123_419_104,
    source_type: SourceType::Wikidata,
    name: "Real-time PCR Data Essential Spreadsheet Format",
    extensions: &["tsv"],
    media_types: &["text/tab-separated-values"],
    internal_signatures: &[],
    related_formats: &[],
};
