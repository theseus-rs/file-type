use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_375: FileType = FileType {
    file_format: &FileFormat {
        id: 375,
        source_type: SourceType::Pronom,
        name: "Microsoft Publisher",
        extensions: &["pub"],
        media_types: &["application/x-mspublisher"],
        signatures: &[],
        related_formats: &[],
    },
};
