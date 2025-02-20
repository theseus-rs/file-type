use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4115982219: FileType = FileType {
    file_format: &FileFormat {
        id: 4_115_982_219,
        source_type: SourceType::Httpd,
        name: "ibm minipay",
        extensions: &["mpy"],
        media_types: &["application/vnd.ibm.minipay"],
        signatures: &[],
        related_formats: &[],
    },
};
