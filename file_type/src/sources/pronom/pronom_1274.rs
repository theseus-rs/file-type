use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1274: FileType = FileType {
    file_format: &FileFormat {
        id: 1_274,
        source_type: SourceType::Pronom,
        name: "Macro Enabled Microsoft Powerpoint",
        extensions: &["pptm"],
        media_types: &["application/vnd.ms-powerpoint.presentation.macroEnabled.12"],
        signatures: &[],
        related_formats: &[],
    },
};
