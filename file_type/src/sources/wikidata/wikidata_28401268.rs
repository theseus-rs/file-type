use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28401268: FileFormat = FileFormat {
    id: 28_401_268,
    source_type: SourceType::Wikidata,
    name: "XIP",
    extensions: &["xip"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
