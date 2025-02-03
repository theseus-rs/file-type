use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_548264: FileFormat = FileFormat {
    id: 548_264,
    source_type: SourceType::Wikidata,
    name: "XML Metadata Interchange",
    extensions: &["xmi"],
    media_types: &["application/vnd.xmi+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
