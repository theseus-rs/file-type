use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127604847: FileFormat = FileFormat {
    id: 127_604_847,
    puid: "wikidata/127604847",
    name: "AMPL data file",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
