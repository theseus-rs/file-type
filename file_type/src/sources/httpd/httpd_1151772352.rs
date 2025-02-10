use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1151772352: FileType = FileType {
    file_format: &FileFormat {
        id: 1_151_772_352,
        source_type: SourceType::Httpd,
        name: "install instructions",
        extensions: &["install"],
        media_types: &["application/x-install-instructions"],
        signatures: &[],
        related_formats: &[],
    },
};
