use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_15631639: FileFormat = FileFormat {
    id: 15_631_639,
    source_type: SourceType::Wikidata,
    name: "qgs",
    extensions: &["qgs"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
