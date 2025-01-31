use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125925165: FileFormat = FileFormat {
    id: 125_925_165,
    puid: "wikidata/125925165",
    name: "Papyrus Base Formula file",
    extensions: &["pbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
