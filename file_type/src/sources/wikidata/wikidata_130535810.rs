use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130535810: FileFormat = FileFormat {
    id: 130_535_810,
    source_type: SourceType::Wikidata,
    name: "PromQL query file format",
    extensions: &["promql"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
