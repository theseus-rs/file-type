use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125808650: FileFormat = FileFormat {
    id: 125_808_650,
    puid: "wikidata/125808650",
    name: "Mnemosyne 2.0 file",
    extensions: &["db"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
