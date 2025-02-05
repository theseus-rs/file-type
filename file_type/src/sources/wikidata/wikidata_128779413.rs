use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128779413: FileFormat = FileFormat {
    id: 128_779_413,
    source_type: SourceType::Wikidata,
    name: "Cryptographic Protocol Shapes Analyzer file",
    extensions: &["cpsa"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
