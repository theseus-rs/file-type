use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110016938: FileFormat = FileFormat {
    id: 110_016_938,
    source_type: SourceType::Wikidata,
    name: "TEI P5 - Single Text File",
    extensions: &["odd", "tei", "xml"],
    media_types: &["application/tei+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
