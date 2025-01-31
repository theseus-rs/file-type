use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27861488: FileFormat = FileFormat {
    id: 27_861_488,
    puid: "wikidata/27861488",
    name: "Renoise Song, version 21",
    extensions: &["xrns"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
