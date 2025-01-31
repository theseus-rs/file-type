use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29905089: FileFormat = FileFormat {
    id: 29_905_089,
    puid: "wikidata/29905089",
    name: "Statistical Analysis System consolidation database file",
    extensions: &["sas7bfdb", "sf2", "sf7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
