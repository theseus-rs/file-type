use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105822756: FileFormat = FileFormat {
    id: 105_822_756,
    puid: "wikidata/105822756",
    name: "AMDIS RI Calibration Data",
    extensions: &["CAL"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
