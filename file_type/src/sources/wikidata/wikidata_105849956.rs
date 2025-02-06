use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849956: FileFormat = FileFormat {
    id: 105_849_956,
    source_type: SourceType::Wikidata,
    name: "16bit DOS COM USCC encrypted",
    extensions: &["com"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
