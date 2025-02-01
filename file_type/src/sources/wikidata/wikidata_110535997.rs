use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110535997: FileFormat = FileFormat {
    id: 110_535_997,
    puid: "wikidata/110535997",
    name: "Movie Magic Screenwriter timed backup document",
    extensions: &["tmb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
