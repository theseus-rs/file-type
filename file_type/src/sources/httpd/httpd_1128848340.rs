use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1128848340: FileType = FileType {
    file_format: &FileFormat {
        id: 1_128_848_340,
        source_type: SourceType::Httpd,
        name: "adpcm",
        extensions: &["adp"],
        media_types: &["audio/adpcm"],
        signatures: &[],
        related_formats: &[],
    },
};
