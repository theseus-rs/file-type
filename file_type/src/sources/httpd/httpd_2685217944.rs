use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2685217944: FileFormat = FileFormat {
    id: 2_685_217_944,
    source_type: SourceType::Httpd,
    name: "palm",
    extensions: &["pdb", "pqa", "oprc"],
    media_types: &["application/vnd.palm"],
    signatures: &[],
    related_formats: &[],
};
