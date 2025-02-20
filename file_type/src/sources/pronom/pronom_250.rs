use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
