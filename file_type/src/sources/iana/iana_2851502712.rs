use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2851502712: FileType = FileType {
    file_format: &FileFormat {
        id: 2_851_502_712,
        source_type: SourceType::Iana,
        name: "vnd.ruckus.download",
        extensions: &[],
        media_types: &["application/vnd.ruckus.download"],
        signatures: &[],
        related_formats: &[],
    },
};
