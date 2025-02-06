use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127604847: FileFormat = FileFormat {
    id: 127_604_847,
    source_type: SourceType::Wikidata,
    name: "AMPL data file",
    extensions: &["dat"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
