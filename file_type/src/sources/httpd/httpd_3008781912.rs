use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3008781912: FileFormat = FileFormat {
    id: 3_008_781_912,
    source_type: SourceType::Httpd,
    name: "oasis opendocument formula template",
    extensions: &["odft"],
    media_types: &["application/vnd.oasis.opendocument.formula-template"],
    signatures: &[],
    related_formats: &[],
};
