use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128594313: FileFormat = FileFormat {
    id: 128_594_313,
    source_type: SourceType::Wikidata,
    name: "Agda file",
    extensions: &["agda"],
    media_types: &["text/x-agda"],
    internal_signatures: &[],
    related_formats: &[],
};
