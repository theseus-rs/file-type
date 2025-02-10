use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1675095755: FileType = FileType {
    file_format: &FileFormat {
        id: 1_675_095_755,
        source_type: SourceType::Iana,
        name: "3gpdash-qoe-report+xml",
        extensions: &[],
        media_types: &["application/3gpdash-qoe-report+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
