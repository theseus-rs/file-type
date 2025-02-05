use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29905089: FileFormat = FileFormat {
    id: 29_905_089,
    source_type: SourceType::Wikidata,
    name: "Statistical Analysis System consolidation database file",
    extensions: &["sas7bfdb", "sf2", "sf7"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
