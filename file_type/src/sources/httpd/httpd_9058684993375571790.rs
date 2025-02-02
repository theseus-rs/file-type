use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_9058684993375571790: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "openofficeorg extension",
    extensions: &["oxt"],
    media_types: &["application/vnd.openofficeorg.extension"],
    internal_signatures: &[],
    related_formats: &[],
};
