use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_307400137: FileFormat = FileFormat {
    id: 307_400_137,
    source_type: SourceType::Httpd,
    name: "igloader",
    extensions: &["igl"],
    media_types: &["application/vnd.igloader"],
    internal_signatures: &[],
    related_formats: &[],
};
