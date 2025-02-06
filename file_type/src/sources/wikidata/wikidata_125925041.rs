use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125925041: FileFormat = FileFormat {
    id: 125_925_041,
    source_type: SourceType::Wikidata,
    name: "Papyrus Document Template",
    extensions: &["pav"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
