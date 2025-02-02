use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128779413: FileFormat = FileFormat {
    id: 128_779_413,
    source_type: SourceType::Wikidata,
    name: "Cryptographic Protocol Shapes Analyzer file",
    extensions: &["cpsa"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
