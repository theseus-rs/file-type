use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_496: FileType = FileType {
    file_format: &FileFormat {
        id: 496,
        source_type: SourceType::Pronom,
        name: "Lotus Approach View File",
        extensions: &["apr"],
        media_types: &["application/vnd.lotus-approach"],
        signatures: &[],
        related_formats: &[],
    },
};
