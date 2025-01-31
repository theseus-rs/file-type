use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27861483: FileFormat = FileFormat {
    id: 27_861_483,
    puid: "wikidata/27861483",
    name: "Renoise Song, version 14",
    extensions: &["xrns"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
