use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114889200: FileFormat = FileFormat {
    id: 114_889_200,
    puid: "wikidata/114889200",
    name: "Scrapbook Factory Deluxe Puzzle file",
    extensions: &["spz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
