use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114093710: FileFormat = FileFormat {
    id: 114_093_710,
    source_type: SourceType::Wikidata,
    name: "Sony SLV File",
    extensions: &["slv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
