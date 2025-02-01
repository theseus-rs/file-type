use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_380665: FileFormat = FileFormat {
    id: 380_665,
    puid: "wikidata/380665",
    name: "PLS",
    extensions: &["pls"],
    media_types: &["audio/x-scpls"],
    internal_signatures: &[],
    related_formats: &[],
};
