use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27861480: FileFormat = FileFormat {
    id: 27_861_480,
    puid: "wikidata/27861480",
    name: "Renoise Song, version 9",
    extensions: &["xrns"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
