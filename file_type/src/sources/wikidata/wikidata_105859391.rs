use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859391: FileFormat = FileFormat {
    id: 105_859_391,
    puid: "wikidata/105859391",
    name: "SlickRun MagicWord Pack",
    extensions: &["qrs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
