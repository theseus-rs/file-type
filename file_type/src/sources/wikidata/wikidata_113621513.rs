use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113621513: FileFormat = FileFormat {
    id: 113_621_513,
    puid: "wikidata/113621513",
    name: "Load Runner Scenario file",
    extensions: &["lrs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
