use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1814942741: FileType = FileType {
    file_format: &FileFormat {
        id: 1_814_942_741,
        source_type: SourceType::Httpd,
        name: "ms powerpoint slide macroenabled 12",
        extensions: &["sldm"],
        media_types: &["application/vnd.ms-powerpoint.slide.macroenabled.12"],
        signatures: &[],
        related_formats: &[],
    },
};
