use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114877622: FileFormat = FileFormat {
    id: 114_877_622,
    puid: "wikidata/114877622",
    name: "Scrapbook Factory Deluxe Family Tree file",
    extensions: &["sft"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
