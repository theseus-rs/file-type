use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_100596946: FileFormat = FileFormat {
    id: 100_596_946,
    source_type: SourceType::Wikidata,
    name: "Minitab Worksheet, version 14",
    extensions: &["mtw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
