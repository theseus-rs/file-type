use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850422: FileFormat = FileFormat {
    id: 105_850_422,
    source_type: SourceType::Wikidata,
    name: "16bit DOS COM DS-COM Crypt protected (v1.27)",
    extensions: &["com"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
