use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114877507: FileFormat = FileFormat {
    id: 114_877_507,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Photo Project file",
    extensions: &["spp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
