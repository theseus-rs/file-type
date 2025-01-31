use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865888: FileFormat = FileFormat {
    id: 105_865_888,
    puid: "wikidata/105865888",
    name: "Gerber Photoplot",
    extensions: &["pho"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
