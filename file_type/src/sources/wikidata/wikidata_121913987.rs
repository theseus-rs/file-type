use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121913987: FileFormat = FileFormat {
    id: 121_913_987,
    puid: "wikidata/121913987",
    name: "Digital Voice File TRC Codec",
    extensions: &["dvf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
