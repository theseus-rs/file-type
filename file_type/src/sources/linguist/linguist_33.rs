use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_33: FileType = FileType {
    file_format: &FileFormat {
        id: 33,
        source_type: SourceType::Linguist,
        name: "Blade",
        extensions: &["blade", "blade.php"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
