use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27861489: FileFormat = FileFormat {
    id: 27_861_489,
    puid: "wikidata/27861489",
    name: "Renoise Song, version 22",
    extensions: &["xrns"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
