use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850841: FileFormat = FileFormat {
    id: 105_850_841,
    puid: "wikidata/105850841",
    name: "Sony Ericsson remote control configuration",
    extensions: &["kcf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
