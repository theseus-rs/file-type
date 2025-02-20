use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_835049151: FileType = FileType {
    file_format: &FileFormat {
        id: 835_049_151,
        source_type: SourceType::Httpd,
        name: "ms powerpoint slideshow macroenabled 12",
        extensions: &["ppsm"],
        media_types: &["application/vnd.ms-powerpoint.slideshow.macroenabled.12"],
        signatures: &[],
        related_formats: &[],
    },
};
