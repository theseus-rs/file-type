use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110017310: FileFormat = FileFormat {
    id: 110_017_310,
    source_type: SourceType::Wikidata,
    name: "TEI P5 XML - Corpus File",
    extensions: &["odd", "tei", "xml"],
    media_types: &["application/tei+xml"],
    signatures: &[],
    related_formats: &[],
};
