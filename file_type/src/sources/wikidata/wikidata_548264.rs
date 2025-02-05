use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_548264: FileFormat = FileFormat {
    id: 548_264,
    source_type: SourceType::Wikidata,
    name: "XML Metadata Interchange",
    extensions: &["xmi"],
    media_types: &["application/vnd.xmi+xml"],
    signatures: &[],
    related_formats: &[],
};
