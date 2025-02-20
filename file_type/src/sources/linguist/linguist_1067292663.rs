use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_1067292663: FileType = FileType {
    file_format: &FileFormat {
        id: 1_067_292_663,
        source_type: SourceType::Linguist,
        name: "Antlers",
        extensions: &["antlers.html", "antlers.php", "antlers.xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
