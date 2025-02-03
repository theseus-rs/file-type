use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_129089513: FileFormat = FileFormat {
    id: 129_089_513,
    source_type: SourceType::Wikidata,
    name: "Embedded Ragel file",
    extensions: &["rl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
