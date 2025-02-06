use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114877598: FileFormat = FileFormat {
    id: 114_877_598,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Trading Card file",
    extensions: &["stc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
