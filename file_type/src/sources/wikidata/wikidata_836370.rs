use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_836370: FileFormat = FileFormat {
    id: 836_370,
    source_type: SourceType::Wikidata,
    name: "OPML",
    extensions: &["opml"],
    media_types: &[
        "application/xml",
        "text/x-opml",
        "text/x-opml+xml",
        "text/xml",
    ],
    signatures: &[],
    related_formats: &[],
};
