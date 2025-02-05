use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1213743: FileFormat = FileFormat {
    id: 1_213_743,
    source_type: SourceType::Wikidata,
    name: "NRG",
    extensions: &["nrg"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
