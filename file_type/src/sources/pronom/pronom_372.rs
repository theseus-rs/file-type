use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_372: FileType = FileType {
    file_format: &FileFormat {
        id: 372,
        source_type: SourceType::Pronom,
        name: "Microsoft Publisher",
        extensions: &["pub"],
        media_types: &["application/x-mspublisher"],
        signatures: &[],
        related_formats: &[],
    },
};
