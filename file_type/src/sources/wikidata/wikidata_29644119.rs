use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29644119: FileFormat = FileFormat {
    id: 29_644_119,
    source_type: SourceType::Wikidata,
    name: "ISO/IEC 8211 Data Descriptive File",
    extensions: &["ddf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
