use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_604279: FileFormat = FileFormat {
    id: 604_279,
    puid: "wikidata/604279",
    name: "Dirac",
    extensions: &["drc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
