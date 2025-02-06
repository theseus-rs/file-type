use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118906275: FileFormat = FileFormat {
    id: 118_906_275,
    source_type: SourceType::Wikidata,
    name: "ASP Configuration file",
    extensions: &["asa"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
