use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27966884: FileFormat = FileFormat {
    id: 27_966_884,
    puid: "wikidata/27966884",
    name: "Direct Stream Digital Audio",
    extensions: &["dsf", "dsflib", "minidsf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
