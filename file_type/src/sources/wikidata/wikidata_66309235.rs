use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66309235: FileFormat = FileFormat {
    id: 66_309_235,
    puid: "wikidata/66309235",
    name: "Access Blank Project Template",
    extensions: &["adn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
