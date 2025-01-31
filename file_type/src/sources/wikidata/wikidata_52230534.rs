use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_52230534: FileFormat = FileFormat {
    id: 52_230_534,
    puid: "wikidata/52230534",
    name: "Polynomial Texture Map",
    extensions: &["ptm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
