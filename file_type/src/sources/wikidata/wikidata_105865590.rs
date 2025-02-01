use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865590: FileFormat = FileFormat {
    id: 105_865_590,
    puid: "wikidata/105865590",
    name: "ProPresenter 6 presentation",
    extensions: &["pro6"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
