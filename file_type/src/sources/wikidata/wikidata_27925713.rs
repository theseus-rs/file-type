use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27925713: FileFormat = FileFormat {
    id: 27_925_713,
    source_type: SourceType::Wikidata,
    name: "DTED Level 1 Gazetteer Primary file",
    extensions: &["gaz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
