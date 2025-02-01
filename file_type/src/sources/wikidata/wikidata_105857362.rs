use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857362: FileFormat = FileFormat {
    id: 105_857_362,
    puid: "wikidata/105857362",
    name: "QMK keymap",
    extensions: &["json"],
    media_types: &["text/json"],
    internal_signatures: &[],
    related_formats: &[],
};
