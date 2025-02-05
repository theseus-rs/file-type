use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120867895: FileFormat = FileFormat {
    id: 120_867_895,
    source_type: SourceType::Wikidata,
    name: "Cumulus Record Exchange File",
    extensions: &["cre"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
