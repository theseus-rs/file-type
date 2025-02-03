use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_108837748: FileFormat = FileFormat {
    id: 108_837_748,
    source_type: SourceType::Wikidata,
    name: "OmniPage Pro Document 11",
    extensions: &["opd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
