use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_5508789: FileFormat = FileFormat {
    id: 5_508_789,
    source_type: SourceType::Wikidata,
    name: "Functional Mock-up Interface",
    extensions: &["fmu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
