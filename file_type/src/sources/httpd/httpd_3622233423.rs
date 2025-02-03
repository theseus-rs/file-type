use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3622233423: FileFormat = FileFormat {
    id: 3_622_233_423,
    source_type: SourceType::Httpd,
    name: "openofficeorg extension",
    extensions: &["oxt"],
    media_types: &["application/vnd.openofficeorg.extension"],
    internal_signatures: &[],
    related_formats: &[],
};
