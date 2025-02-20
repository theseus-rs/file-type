use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_132: FileType = FileType {
    file_format: &FileFormat {
        id: 132,
        source_type: SourceType::Pronom,
        name: "Microsoft Powerpoint Presentation Show",
        extensions: &["pps"],
        media_types: &["application/vnd.ms-powerpoint"],
        signatures: &[],
        related_formats: &[],
    },
};
