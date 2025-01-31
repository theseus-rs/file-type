use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27966915: FileFormat = FileFormat {
    id: 27_966_915,
    puid: "wikidata/27966915",
    name: "NES Sound Format Extended",
    extensions: &["nsfe"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
