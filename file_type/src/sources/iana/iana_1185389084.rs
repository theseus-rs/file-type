use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1185389084: FileFormat = FileFormat {
    id: 1_185_389_084,
    source_type: SourceType::Iana,
    name: "vnd.spotfire.dxp",
    extensions: &[],
    media_types: &["application/vnd.spotfire.dxp"],
    internal_signatures: &[],
    related_formats: &[],
};
