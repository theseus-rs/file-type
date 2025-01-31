use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130603160: FileFormat = FileFormat {
    id: 130_603_160,
    puid: "wikidata/130603160",
    name: "REBOL file format",
    extensions: &["r", "r3", "reb"],
    media_types: &["text/x-rebol", "text/x-rebol", "text/x-rebol"],
    internal_signatures: &[],
    related_formats: &[],
};
