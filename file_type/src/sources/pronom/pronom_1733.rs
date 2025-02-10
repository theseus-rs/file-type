use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1733: FileType = FileType {
    file_format: &FileFormat {
        id: 1_733,
        source_type: SourceType::Pronom,
        name: "Microsoft Visio Macro-Enabled Stencil",
        extensions: &["vssm"],
        media_types: &["application/vnd.ms-visio.stencil.macroEnabled.main+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
