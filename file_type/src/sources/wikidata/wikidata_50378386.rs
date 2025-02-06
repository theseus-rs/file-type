use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50378386: FileFormat = FileFormat {
    id: 50_378_386,
    source_type: SourceType::Wikidata,
    name: "INTERLIS Model File, version 1",
    extensions: &["ili"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
