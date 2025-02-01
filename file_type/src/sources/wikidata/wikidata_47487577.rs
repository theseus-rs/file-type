use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47487577: FileFormat = FileFormat {
    id: 47_487_577,
    puid: "wikidata/47487577",
    name: "Alias Scene Description Language",
    extensions: &["sdl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
