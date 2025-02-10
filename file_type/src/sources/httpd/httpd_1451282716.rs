use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1451282716: FileType = FileType {
    file_format: &FileFormat {
        id: 1_451_282_716,
        source_type: SourceType::Httpd,
        name: "stardivision writer global",
        extensions: &["sgl"],
        media_types: &["application/vnd.stardivision.writer-global"],
        signatures: &[],
        related_formats: &[],
    },
};
