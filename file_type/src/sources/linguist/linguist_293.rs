use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_293: FileType = FileType {
    file_format: &FileFormat {
        id: 293,
        source_type: SourceType::Linguist,
        name: "PowerShell",
        extensions: &["ps1", "psd1", "psm1"],
        media_types: &["application/x-powershell"],
        signatures: &[],
        related_formats: &[],
    },
};
