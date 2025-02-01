use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_62561275: FileFormat = FileFormat {
    id: 62_561_275,
    puid: "wikidata/62561275",
    name: "Hangul Word Processor Document, version 5",
    extensions: &["hwp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
