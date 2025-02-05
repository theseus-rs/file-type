use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3523163259: FileFormat = FileFormat {
    id: 3_523_163_259,
    source_type: SourceType::Httpd,
    name: "insors igm",
    extensions: &["igm"],
    media_types: &["application/vnd.insors.igm"],
    signatures: &[],
    related_formats: &[],
};
