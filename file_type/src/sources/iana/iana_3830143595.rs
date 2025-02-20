use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3830143595: FileType = FileType {
    file_format: &FileFormat {
        id: 3_830_143_595,
        source_type: SourceType::Iana,
        name: "yaml",
        extensions: &[],
        media_types: &["application/yaml"],
        signatures: &[],
        related_formats: &[],
    },
};
