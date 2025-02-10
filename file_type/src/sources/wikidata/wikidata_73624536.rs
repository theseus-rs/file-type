use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_73624536: FileType = FileType {
    file_format: &FileFormat {
        id: 73_624_536,
        source_type: SourceType::Wikidata,
        name: "Intuit QuickBooks Backup",
        extensions: &["qbb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
