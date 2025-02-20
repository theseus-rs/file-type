use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_899: FileType = FileType {
    file_format: &FileFormat {
        id: 899,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Database for DOS",
        extensions: &["wdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
