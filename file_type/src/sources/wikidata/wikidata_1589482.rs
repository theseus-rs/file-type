use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1589482: FileFormat = FileFormat {
    id: 1_589_482,
    source_type: SourceType::Wikidata,
    name: "JT",
    extensions: &["JT"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
