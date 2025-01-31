use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_5508789: FileFormat = FileFormat {
    id: 5_508_789,
    puid: "wikidata/5508789",
    name: "Functional Mock-up Interface",
    extensions: &["fmu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
