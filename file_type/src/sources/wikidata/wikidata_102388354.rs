use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_102388354: FileFormat = FileFormat {
    id: 102_388_354,
    source_type: SourceType::Wikidata,
    name: "SPARQL-Generate",
    extensions: &["rqg"],
    media_types: &["application/vnd.sparql-generate"],
    internal_signatures: &[],
    related_formats: &[],
};
