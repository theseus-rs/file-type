use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1800243101: FileType = FileType {
    file_format: &FileFormat {
        id: 1_800_243_101,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.mcvideo-mbms-usage-info+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.mcvideo-mbms-usage-info+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
