use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112653540: FileFormat = FileFormat {
    id: 112_653_540,
    source_type: SourceType::Wikidata,
    name: "WebScan Files",
    extensions: &["wsc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
