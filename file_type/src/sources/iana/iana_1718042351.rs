use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1718042351: FileType = FileType {
    file_format: &FileFormat {
        id: 1_718_042_351,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.pfcp",
        extensions: &[],
        media_types: &["application/vnd.3gpp.pfcp"],
        signatures: &[],
        related_formats: &[],
    },
};
