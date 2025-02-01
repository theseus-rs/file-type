use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125925041: FileFormat = FileFormat {
    id: 125_925_041,
    puid: "wikidata/125925041",
    name: "Papyrus Document Template",
    extensions: &["pav"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
