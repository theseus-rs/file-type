use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4263354093: FileType = FileType {
    file_format: &FileFormat {
        id: 4_263_354_093,
        source_type: SourceType::Iana,
        name: "jxsv",
        extensions: &[],
        media_types: &["video/jxsv"],
        signatures: &[],
        related_formats: &[],
    },
};
