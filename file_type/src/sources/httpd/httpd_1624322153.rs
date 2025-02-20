use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1624322153: FileType = FileType {
    file_format: &FileFormat {
        id: 1_624_322_153,
        source_type: SourceType::Httpd,
        name: "ibm modcap",
        extensions: &["afp", "listafp", "list3820"],
        media_types: &["application/vnd.ibm.modcap"],
        signatures: &[],
        related_formats: &[],
    },
};
