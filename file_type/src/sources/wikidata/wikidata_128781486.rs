use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128781486: FileFormat = FileFormat {
    id: 128_781_486,
    puid: "wikidata/128781486",
    name: "Cryptol file format",
    extensions: &["cry"],
    media_types: &["text/x-cryptol"],
    internal_signatures: &[],
    related_formats: &[],
};
