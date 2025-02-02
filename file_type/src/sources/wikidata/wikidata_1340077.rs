use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1340077: FileFormat = FileFormat {
    id: 1_340_077,
    source_type: SourceType::Wikidata,
    name: "Encoded Archival Description",
    extensions: &["sgm", "sgml"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
