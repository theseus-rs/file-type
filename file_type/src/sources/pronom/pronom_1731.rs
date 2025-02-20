use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1731: FileType = FileType {
    file_format: &FileFormat {
        id: 1_731,
        source_type: SourceType::Pronom,
        name: "Microsoft Visio Template",
        extensions: &["vstx"],
        media_types: &["application/vnd.ms-visio.template.main+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
