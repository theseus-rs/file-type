use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_106410286: FileFormat = FileFormat {
    id: 106_410_286,
    source_type: SourceType::Wikidata,
    name: "ZISRAW (CZI)",
    extensions: &["czi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
