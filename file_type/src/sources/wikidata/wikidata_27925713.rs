use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27925713: FileFormat = FileFormat {
    id: 27_925_713,
    source_type: SourceType::Wikidata,
    name: "DTED Level 1 Gazetteer Primary file",
    extensions: &["gaz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
