use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112653540: FileFormat = FileFormat {
    id: 112_653_540,
    source_type: SourceType::Wikidata,
    name: "WebScan Files",
    extensions: &["wsc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
