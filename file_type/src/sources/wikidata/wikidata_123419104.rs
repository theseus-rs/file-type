use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123419104: FileFormat = FileFormat {
    id: 123_419_104,
    puid: "wikidata/123419104",
    name: "Real-time PCR Data Essential Spreadsheet Format",
    extensions: &["tsv"],
    media_types: &["text/tab-separated-values"],
    internal_signatures: &[],
    related_formats: &[],
};
