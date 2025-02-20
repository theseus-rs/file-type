use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1427603842: FileType = FileType {
    file_format: &FileFormat {
        id: 1_427_603_842,
        source_type: SourceType::Iana,
        name: "vnd.heroku+json",
        extensions: &[],
        media_types: &["application/vnd.heroku+json"],
        signatures: &[],
        related_formats: &[],
    },
};
