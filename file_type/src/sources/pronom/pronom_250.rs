use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_250: FileType = FileType {
    file_format: &FileFormat {
        id: 250,
        source_type: SourceType::Pronom,
        name: "Microsoft PowerPoint Graphics File",
        extensions: &["ppi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
