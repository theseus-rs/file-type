use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130535810: FileFormat = FileFormat {
    id: 130_535_810,
    source_type: SourceType::Wikidata,
    name: "PromQL query file format",
    extensions: &["promql"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
