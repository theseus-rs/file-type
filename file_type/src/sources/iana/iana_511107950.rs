use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_511107950: FileType = FileType {
    file_format: &FileFormat {
        id: 511_107_950,
        source_type: SourceType::Iana,
        name: "vnd.nearst.inv+json",
        extensions: &[],
        media_types: &["application/vnd.nearst.inv+json"],
        signatures: &[],
        related_formats: &[],
    },
};
