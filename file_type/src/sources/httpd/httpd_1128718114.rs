use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1128718114: FileType = FileType {
    file_format: &FileFormat {
        id: 1_128_718_114,
        source_type: SourceType::Httpd,
        name: "mobius plc",
        extensions: &["plc"],
        media_types: &["application/vnd.mobius.plc"],
        signatures: &[],
        related_formats: &[],
    },
};
