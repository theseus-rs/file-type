use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1800969323: FileFormat = FileFormat {
    id: 1_800_969_323,
    source_type: SourceType::Iana,
    name: "vnd.ecowin.seriesrequest",
    extensions: &[],
    media_types: &["application/vnd.ecowin.seriesrequest"],
    signatures: &[],
    related_formats: &[],
};
