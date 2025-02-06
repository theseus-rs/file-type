use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849853: FileFormat = FileFormat {
    id: 105_849_853,
    source_type: SourceType::Wikidata,
    name: "16bit DOS COM DS-COM Crypt protected (v1.31)",
    extensions: &["com"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
