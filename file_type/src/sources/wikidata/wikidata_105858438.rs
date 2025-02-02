use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858438: FileFormat = FileFormat {
    id: 105_858_438,
    source_type: SourceType::Wikidata,
    name: "ESU electronic sounds",
    extensions: &["esu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
