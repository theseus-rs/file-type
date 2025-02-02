use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_2313301: FileFormat = FileFormat {
    id: 2_313_301,
    source_type: SourceType::Wikidata,
    name: "SpreadsheetML",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
