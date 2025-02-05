use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110016211: FileFormat = FileFormat {
    id: 110_016_211,
    source_type: SourceType::Wikidata,
    name: "TEI P4 XML - Single Text File",
    extensions: &["odd", "tei", "xml"],
    media_types: &["application/tei+xml"],
    signatures: &[],
    related_formats: &[],
};
