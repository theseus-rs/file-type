use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1732: FileType = FileType {
    file_format: &FileFormat {
        id: 1_732,
        source_type: SourceType::Pronom,
        name: "Microsoft Visio Macro-Enabled Drawing",
        extensions: &["vsdm"],
        media_types: &["application/vnd.ms-visio.drawing.macroEnabled.main+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
