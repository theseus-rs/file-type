use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_62522682: FileFormat = FileFormat {
    id: 62_522_682,
    source_type: SourceType::Wikidata,
    name: "SPARQL update",
    extensions: &["ru"],
    media_types: &["application/sparql-update"],
    signatures: &[],
    related_formats: &[],
};
