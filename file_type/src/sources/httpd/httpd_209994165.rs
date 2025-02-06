use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_209994165: FileFormat = FileFormat {
    id: 209_994_165,
    source_type: SourceType::Httpd,
    name: "set registration initiation",
    extensions: &["setreg"],
    media_types: &["application/set-registration-initiation"],
    signatures: &[],
    related_formats: &[],
};
