use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1122457956: FileType = FileType {
    file_format: &FileFormat {
        id: 1_122_457_956,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.seal-mbms-usage-info+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.seal-mbms-usage-info+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
