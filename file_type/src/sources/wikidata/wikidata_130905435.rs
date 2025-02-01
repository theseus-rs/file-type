use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130905435: FileFormat = FileFormat {
    id: 130_905_435,
    puid: "wikidata/130905435",
    name: "Sieve file format",
    extensions: &["sieve", "siv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
