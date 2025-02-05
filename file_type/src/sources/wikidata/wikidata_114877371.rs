use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114877371: FileFormat = FileFormat {
    id: 114_877_371,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Scrapbook file",
    extensions: &["sbk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
