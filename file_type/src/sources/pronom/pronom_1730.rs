use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1730: FileType = FileType {
    file_format: &FileFormat {
        id: 1_730,
        source_type: SourceType::Pronom,
        name: "Microsoft Visio Stencil",
        extensions: &["vssx"],
        media_types: &["application/vnd.ms-visio.stencil.main+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
