use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_44044: FileFormat = FileFormat {
    id: 44_044,
    source_type: SourceType::Wikidata,
    name: "N-Triples",
    extensions: &["nt"],
    media_types: &["application/n-triples"],
    internal_signatures: &[],
    related_formats: &[],
};
