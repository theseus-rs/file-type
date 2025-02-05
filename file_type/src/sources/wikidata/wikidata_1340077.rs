use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1340077: FileFormat = FileFormat {
    id: 1_340_077,
    source_type: SourceType::Wikidata,
    name: "Encoded Archival Description",
    extensions: &["sgm", "sgml"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
