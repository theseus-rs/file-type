use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967214: FileFormat = FileFormat {
    id: 27_967_214,
    source_type: SourceType::Wikidata,
    name: "SBStudio module",
    extensions: &["pac", "son", "sou"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
