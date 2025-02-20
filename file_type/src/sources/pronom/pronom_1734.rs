use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1734: FileType = FileType {
    file_format: &FileFormat {
        id: 1_734,
        source_type: SourceType::Pronom,
        name: "Microsoft Visio Macro-Enabled Template",
        extensions: &["vstm"],
        media_types: &["application/vnd.ms-visio.template.macroEnabled.main+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
