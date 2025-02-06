use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_62561203: FileFormat = FileFormat {
    id: 62_561_203,
    source_type: SourceType::Wikidata,
    name: "Corel Presentation",
    extensions: &["shw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
