use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27861490: FileFormat = FileFormat {
    id: 27_861_490,
    puid: "wikidata/27861490",
    name: "Renoise Song, version 30",
    extensions: &["xrns"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
