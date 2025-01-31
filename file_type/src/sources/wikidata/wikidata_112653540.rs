use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112653540: FileFormat = FileFormat {
    id: 112_653_540,
    puid: "wikidata/112653540",
    name: "WebScan Files",
    extensions: &["wsc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
