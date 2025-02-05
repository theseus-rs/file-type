use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125592921: FileFormat = FileFormat {
    id: 125_592_921,
    source_type: SourceType::Wikidata,
    name: "Raw CMYK",
    extensions: &["cmyk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
