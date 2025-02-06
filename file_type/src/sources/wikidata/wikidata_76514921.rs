use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_76514921: FileFormat = FileFormat {
    id: 76_514_921,
    source_type: SourceType::Wikidata,
    name: "WinDev Window",
    extensions: &["wdw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
