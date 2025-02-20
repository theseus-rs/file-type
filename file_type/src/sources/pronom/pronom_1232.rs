use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1232: FileType = FileType {
    file_format: &FileFormat {
        id: 1_232,
        source_type: SourceType::Pronom,
        name: "Microsoft Excel Macro-Enabled",
        extensions: &["xlsm"],
        media_types: &["application/vnd.ms-excel.sheet.macroEnabled.12"],
        signatures: &[],
        related_formats: &[],
    },
};
