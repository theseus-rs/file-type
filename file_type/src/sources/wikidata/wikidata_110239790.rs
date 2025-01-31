use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110239790: FileFormat = FileFormat {
    id: 110_239_790,
    puid: "wikidata/110239790",
    name: "JData",
    extensions: &["jdb", "jdb", "jdt", "jdt"],
    media_types: &[
        "application/jdata-binary",
        "application/jdata-binary",
        "application/jdata-text",
        "application/jdata-text",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
