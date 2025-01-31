use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128779413: FileFormat = FileFormat {
    id: 128_779_413,
    puid: "wikidata/128779413",
    name: "Cryptographic Protocol Shapes Analyzer file",
    extensions: &["cpsa"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
