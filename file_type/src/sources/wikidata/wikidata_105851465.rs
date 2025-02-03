use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851465: FileFormat = FileFormat {
    id: 105_851_465,
    source_type: SourceType::Wikidata,
    name: "Garmin Training Center Database XML (V2)",
    extensions: &["tcx"],
    media_types: &["application/vnd.garmin.tcx+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
