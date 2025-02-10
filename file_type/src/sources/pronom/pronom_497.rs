use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_497: FileType = FileType {
    file_format: &FileFormat {
        id: 497,
        source_type: SourceType::Pronom,
        name: "Lotus Approach View File",
        extensions: &["apt"],
        media_types: &["application/vnd.lotus-approach"],
        signatures: &[],
        related_formats: &[],
    },
};
