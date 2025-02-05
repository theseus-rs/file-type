use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2298246905: FileFormat = FileFormat {
    id: 2_298_246_905,
    source_type: SourceType::Iana,
    name: "vnd.fdsn.stationxml+xml",
    extensions: &[],
    media_types: &["application/vnd.fdsn.stationxml+xml"],
    signatures: &[],
    related_formats: &[],
};
