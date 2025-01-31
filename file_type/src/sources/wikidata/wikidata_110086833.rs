use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110086833: FileFormat = FileFormat {
    id: 110_086_833,
    puid: "wikidata/110086833",
    name: "Agisoft Tiled Model",
    extensions: &["tls"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
