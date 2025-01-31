use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_836370: FileFormat = FileFormat {
    id: 836_370,
    puid: "wikidata/836370",
    name: "OPML",
    extensions: &["opml", "opml", "opml", "opml"],
    media_types: &[
        "application/xml",
        "text/x-opml",
        "text/x-opml+xml",
        "text/xml",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
