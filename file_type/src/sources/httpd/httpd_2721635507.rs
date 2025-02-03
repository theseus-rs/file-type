use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2721635507: FileFormat = FileFormat {
    id: 2_721_635_507,
    source_type: SourceType::Httpd,
    name: "micrografx igx",
    extensions: &["igx"],
    media_types: &["application/vnd.micrografx.igx"],
    internal_signatures: &[],
    related_formats: &[],
};
