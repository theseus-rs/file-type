use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123378395: FileFormat = FileFormat {
    id: 123_378_395,
    puid: "wikidata/123378395",
    name: "Radiosity Solution file",
    extensions: &["lwr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
