use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105822756: FileFormat = FileFormat {
    id: 105_822_756,
    source_type: SourceType::Wikidata,
    name: "AMDIS RI Calibration Data",
    extensions: &["CAL"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
