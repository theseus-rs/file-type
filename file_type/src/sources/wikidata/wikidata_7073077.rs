use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7073077: FileFormat = FileFormat {
    id: 7_073_077,
    puid: "wikidata/7073077",
    name: "OTA bitmap",
    extensions: &["otb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
