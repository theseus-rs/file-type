use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100596960: FileFormat = FileFormat {
    id: 100_596_960,
    puid: "wikidata/100596960",
    name: "Minitab Worksheet, version 15+",
    extensions: &["mtw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
