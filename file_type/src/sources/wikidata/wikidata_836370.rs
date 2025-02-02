use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

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
    internal_signatures: &[],
    related_formats: &[],
};
