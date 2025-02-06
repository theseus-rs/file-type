use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1884967282: FileFormat = FileFormat {
    id: 1_884_967_282,
    source_type: SourceType::Iana,
    name: "vnd.snesdev-page-table",
    extensions: &[],
    media_types: &["application/vnd.snesdev-page-table"],
    signatures: &[],
    related_formats: &[],
};
