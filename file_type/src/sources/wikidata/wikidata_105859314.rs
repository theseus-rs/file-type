use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859314: FileFormat = FileFormat {
    id: 105_859_314,
    puid: "wikidata/105859314",
    name: "QuickBooks Web Connector configuration",
    extensions: &["qwc"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
