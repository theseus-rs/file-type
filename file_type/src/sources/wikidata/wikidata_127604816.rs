use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127604816: FileFormat = FileFormat {
    id: 127_604_816,
    source_type: SourceType::Wikidata,
    name: "AMPL model file",
    extensions: &["mod"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
