use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125592921: FileFormat = FileFormat {
    id: 125_592_921,
    source_type: SourceType::Wikidata,
    name: "Raw CMYK",
    extensions: &["cmyk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
