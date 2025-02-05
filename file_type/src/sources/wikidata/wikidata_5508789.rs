use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_5508789: FileFormat = FileFormat {
    id: 5_508_789,
    source_type: SourceType::Wikidata,
    name: "Functional Mock-up Interface",
    extensions: &["fmu"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
