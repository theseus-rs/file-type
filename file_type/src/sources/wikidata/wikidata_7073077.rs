use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_7073077: FileFormat = FileFormat {
    id: 7_073_077,
    source_type: SourceType::Wikidata,
    name: "OTA bitmap",
    extensions: &["otb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
