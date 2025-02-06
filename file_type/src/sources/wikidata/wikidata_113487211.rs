use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113487211: FileFormat = FileFormat {
    id: 113_487_211,
    source_type: SourceType::Wikidata,
    name: "Persuasion Windows Document 3 - 4",
    extensions: &["at3", "at4", "pn4", "pr3"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
