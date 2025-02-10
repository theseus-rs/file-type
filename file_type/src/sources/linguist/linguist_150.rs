use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_150: FileType = FileType {
    file_format: &FileFormat {
        id: 150,
        source_type: SourceType::Linguist,
        name: "HTML+ERB",
        extensions: &["erb", "erb.deface", "rhtml"],
        media_types: &["application/x-erb"],
        signatures: &[],
        related_formats: &[],
    },
};
