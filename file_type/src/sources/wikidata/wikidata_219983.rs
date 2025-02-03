use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_219983: FileFormat = FileFormat {
    id: 219_983,
    source_type: SourceType::Wikidata,
    name: "Zoo",
    extensions: &["zoo"],
    media_types: &["application/x-zoo"],
    internal_signatures: &[],
    related_formats: &[],
};
