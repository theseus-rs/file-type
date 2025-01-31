use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118906275: FileFormat = FileFormat {
    id: 118_906_275,
    puid: "wikidata/118906275",
    name: "ASP Configuration file",
    extensions: &["asa"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
