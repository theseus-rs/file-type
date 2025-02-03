use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3148081406512757481: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "set registration initiation",
    extensions: &["setreg"],
    media_types: &["application/set-registration-initiation"],
    internal_signatures: &[],
    related_formats: &[],
};
