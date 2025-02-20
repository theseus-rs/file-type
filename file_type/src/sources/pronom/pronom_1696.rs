use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1696: FileType = FileType {
    file_format: &FileFormat {
        id: 1_696,
        source_type: SourceType::Pronom,
        name: "Compound WordPerfect for Windows Document",
        extensions: &["wpd", "doc", "wp6", "wp", "w60"],
        media_types: &["application/vnd.wordperfect"],
        signatures: &[],
        related_formats: &[],
    },
};
