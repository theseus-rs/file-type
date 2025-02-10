use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_370: FileType = FileType {
    file_format: &FileFormat {
        id: 370,
        source_type: SourceType::Pronom,
        name: "Microsoft Publisher",
        extensions: &["pub"],
        media_types: &["application/x-mspublisher"],
        signatures: &[],
        related_formats: &[],
    },
};
