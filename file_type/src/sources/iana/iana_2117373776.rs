use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2117373776: FileType = FileType {
    file_format: &FileFormat {
        id: 2_117_373_776,
        source_type: SourceType::Iana,
        name: "vnd.android.ota",
        extensions: &[],
        media_types: &["application/vnd.android.ota"],
        signatures: &[],
        related_formats: &[],
    },
};
