use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_62625183: FileType = FileType {
    file_format: &FileFormat {
        id: 62_625_183,
        source_type: SourceType::Wikidata,
        name: "PowerShell script",
        extensions: &["ps1", "psm1"],
        media_types: &["application/x-powershell", "text/x-powershell"],
        signatures: &[],
        related_formats: &[],
    },
};
