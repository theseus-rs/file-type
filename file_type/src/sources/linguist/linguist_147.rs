use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_147: FileType = FileType {
    file_format: &FileFormat {
        id: 147,
        source_type: SourceType::Linguist,
        name: "Jinja",
        extensions: &["j2", "jinja", "jinja2"],
        media_types: &["text/x-django"],
        signatures: &[],
        related_formats: &[],
    },
};
