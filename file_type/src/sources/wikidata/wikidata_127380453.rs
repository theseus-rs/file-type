use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127380453: FileFormat = FileFormat {
    id: 127_380_453,
    source_type: SourceType::Wikidata,
    name: "Vulnerability Exploitability eXchange file",
    extensions: &["vex"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
