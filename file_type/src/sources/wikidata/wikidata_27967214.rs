use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967214: FileFormat = FileFormat {
    id: 27_967_214,
    source_type: SourceType::Wikidata,
    name: "SBStudio module",
    extensions: &["pac", "son", "sou"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
