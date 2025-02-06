use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_106410286: FileFormat = FileFormat {
    id: 106_410_286,
    source_type: SourceType::Wikidata,
    name: "ZISRAW (CZI)",
    extensions: &["czi"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
