use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113532977: FileFormat = FileFormat {
    id: 113_532_977,
    source_type: SourceType::Wikidata,
    name: "Wordcraft Chapter File",
    extensions: &["001"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
