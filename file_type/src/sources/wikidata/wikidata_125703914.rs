use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125703914: FileFormat = FileFormat {
    id: 125_703_914,
    puid: "wikidata/125703914",
    name: "StarWriter Graphics Format",
    extensions: &["sgf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
