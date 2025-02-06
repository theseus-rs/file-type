use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112581715: FileFormat = FileFormat {
    id: 112_581_715,
    source_type: SourceType::Wikidata,
    name: "WAN",
    extensions: &["wan"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
