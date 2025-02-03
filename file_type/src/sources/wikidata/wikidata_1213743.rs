use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1213743: FileFormat = FileFormat {
    id: 1_213_743,
    source_type: SourceType::Wikidata,
    name: "NRG",
    extensions: &["nrg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
