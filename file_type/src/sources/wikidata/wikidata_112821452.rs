use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112821452: FileFormat = FileFormat {
    id: 112_821_452,
    puid: "wikidata/112821452",
    name: "Pro/ENGINEER rendering data file",
    extensions: &["slp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
