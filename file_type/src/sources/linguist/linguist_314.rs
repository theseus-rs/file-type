use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_314: FileType = FileType {
    file_format: &FileFormat {
        id: 314,
        source_type: SourceType::Linguist,
        name: "RPM Spec",
        extensions: &["spec"],
        media_types: &["text/x-rpm-spec"],
        signatures: &[],
        related_formats: &[],
    },
};
