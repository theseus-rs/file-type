use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27861478: FileFormat = FileFormat {
    id: 27_861_478,
    puid: "wikidata/27861478",
    name: "Renoise Song, version 4",
    extensions: &["xrns"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
