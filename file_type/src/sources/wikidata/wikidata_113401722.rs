use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113401722: FileFormat = FileFormat {
    id: 113_401_722,
    source_type: SourceType::Wikidata,
    name: "Linux/i386 Binary Executable File ZMAGIC",
    extensions: &["o", "so"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
