use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_60413560: FileFormat = FileFormat {
    id: 60_413_560,
    puid: "wikidata/60413560",
    name: "INTERLIS Transfer File, version 2.3",
    extensions: &["xtf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
