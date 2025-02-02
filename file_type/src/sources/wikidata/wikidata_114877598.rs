use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114877598: FileFormat = FileFormat {
    id: 114_877_598,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Trading Card file",
    extensions: &["stc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
