use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127380453: FileFormat = FileFormat {
    id: 127_380_453,
    puid: "wikidata/127380453",
    name: "Vulnerability Exploitability eXchange file",
    extensions: &["vex"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
