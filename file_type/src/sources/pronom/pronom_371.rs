use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_371: FileType = FileType {
    file_format: &FileFormat {
        id: 371,
        source_type: SourceType::Pronom,
        name: "Microsoft Publisher",
        extensions: &["pub"],
        media_types: &["application/x-mspublisher"],
        signatures: &[],
        related_formats: &[],
    },
};
