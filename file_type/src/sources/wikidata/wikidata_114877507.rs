use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114877507: FileFormat = FileFormat {
    id: 114_877_507,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Photo Project file",
    extensions: &["spp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
