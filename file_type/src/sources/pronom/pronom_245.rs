use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_245: FileType = FileType {
    file_format: &FileFormat {
        id: 245,
        source_type: SourceType::Pronom,
        name: "Microsoft FoxPro Library",
        extensions: &["plb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
