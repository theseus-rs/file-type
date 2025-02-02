use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_293: FileFormat = FileFormat {
    id: 293,
    source_type: SourceType::Linguist,
    name: "PowerShell",
    extensions: &["ps1", "psd1", "psm1"],
    media_types: &["application/x-powershell"],
    internal_signatures: &[],
    related_formats: &[],
};
