use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_2313301: FileFormat = FileFormat {
    id: 2_313_301,
    source_type: SourceType::Wikidata,
    name: "SpreadsheetML",
    extensions: &["xml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
