use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_108837748: FileFormat = FileFormat {
    id: 108_837_748,
    source_type: SourceType::Wikidata,
    name: "OmniPage Pro Document 11",
    extensions: &["opd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
