use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114877598: FileFormat = FileFormat {
    id: 114_877_598,
    puid: "wikidata/114877598",
    name: "Scrapbook Factory Deluxe Trading Card file",
    extensions: &["stc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
