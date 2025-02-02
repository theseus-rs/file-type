use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859314: FileFormat = FileFormat {
    id: 105_859_314,
    source_type: SourceType::Wikidata,
    name: "QuickBooks Web Connector configuration",
    extensions: &["qwc"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
