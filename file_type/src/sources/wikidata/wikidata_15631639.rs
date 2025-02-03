use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_15631639: FileFormat = FileFormat {
    id: 15_631_639,
    source_type: SourceType::Wikidata,
    name: "qgs",
    extensions: &["qgs"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
