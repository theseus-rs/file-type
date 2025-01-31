use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100596946: FileFormat = FileFormat {
    id: 100_596_946,
    puid: "wikidata/100596946",
    name: "Minitab Worksheet, version 14",
    extensions: &["mtw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
