use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1541299051: FileFormat = FileFormat {
    id: 1_541_299_051,
    source_type: SourceType::Httpd,
    name: "oasis opendocument image template",
    extensions: &["oti"],
    media_types: &["application/vnd.oasis.opendocument.image-template"],
    signatures: &[],
    related_formats: &[],
};
