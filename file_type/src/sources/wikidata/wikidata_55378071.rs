use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_55378071: FileFormat = FileFormat {
    id: 55_378_071,
    puid: "wikidata/55378071",
    name: "Marvin Document format",
    extensions: &["mrv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
