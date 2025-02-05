use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_100424447: FileFormat = FileFormat {
    id: 100_424_447,
    source_type: SourceType::Wikidata,
    name: "Minitab Worksheet, version 8-11",
    extensions: &["mtw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
