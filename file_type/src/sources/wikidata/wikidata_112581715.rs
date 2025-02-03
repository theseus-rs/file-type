use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112581715: FileFormat = FileFormat {
    id: 112_581_715,
    source_type: SourceType::Wikidata,
    name: "WAN",
    extensions: &["wan"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
