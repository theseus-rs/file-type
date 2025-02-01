use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128594313: FileFormat = FileFormat {
    id: 128_594_313,
    puid: "wikidata/128594313",
    name: "Agda file",
    extensions: &["agda"],
    media_types: &["text/x-agda"],
    internal_signatures: &[],
    related_formats: &[],
};
