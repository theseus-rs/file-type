use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1524: FileType = FileType {
    file_format: &FileFormat {
        id: 1_524,
        source_type: SourceType::Pronom,
        name: "Microsoft Project",
        extensions: &["mpp"],
        media_types: &["application/vnd.ms-project"],
        signatures: &[],
        related_formats: &[],
    },
};
