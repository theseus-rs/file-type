use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_16: FileType = FileType {
    file_format: &FileFormat {
        id: 16,
        source_type: SourceType::Linguist,
        name: "ApacheConf",
        extensions: &["apacheconf", "vhost"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
