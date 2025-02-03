use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3629759553: FileFormat = FileFormat {
    id: 3_629_759_553,
    source_type: SourceType::Httpd,
    name: "dvi",
    extensions: &["dvi"],
    media_types: &["application/x-dvi"],
    internal_signatures: &[],
    related_formats: &[],
};
