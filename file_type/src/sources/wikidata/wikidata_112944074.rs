use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112944074: FileFormat = FileFormat {
    id: 112_944_074,
    puid: "wikidata/112944074",
    name: "GameExchange2 skeleton file",
    extensions: &["GSF"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
