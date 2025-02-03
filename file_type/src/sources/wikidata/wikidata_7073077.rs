use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_7073077: FileFormat = FileFormat {
    id: 7_073_077,
    source_type: SourceType::Wikidata,
    name: "OTA bitmap",
    extensions: &["otb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
