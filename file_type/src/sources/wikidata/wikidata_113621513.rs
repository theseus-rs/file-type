use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113621513: FileFormat = FileFormat {
    id: 113_621_513,
    source_type: SourceType::Wikidata,
    name: "Load Runner Scenario file",
    extensions: &["lrs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
