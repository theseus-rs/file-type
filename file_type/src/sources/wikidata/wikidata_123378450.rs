use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123378450: FileFormat = FileFormat {
    id: 123_378_450,
    source_type: SourceType::Wikidata,
    name: "TrueSpace Selection file",
    extensions: &["sel"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
