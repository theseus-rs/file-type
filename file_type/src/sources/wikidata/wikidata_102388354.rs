use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_102388354: FileFormat = FileFormat {
    id: 102_388_354,
    source_type: SourceType::Wikidata,
    name: "SPARQL-Generate",
    extensions: &["rqg"],
    media_types: &["application/vnd.sparql-generate"],
    signatures: &[],
    related_formats: &[],
};
