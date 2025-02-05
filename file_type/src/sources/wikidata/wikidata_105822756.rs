use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105822756: FileFormat = FileFormat {
    id: 105_822_756,
    source_type: SourceType::Wikidata,
    name: "AMDIS RI Calibration Data",
    extensions: &["CAL"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
