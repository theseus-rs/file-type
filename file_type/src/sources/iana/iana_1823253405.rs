use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1823253405: FileType = FileType {
    file_format: &FileFormat {
        id: 1_823_253_405,
        source_type: SourceType::Iana,
        name: "vnd.orange.indata",
        extensions: &[],
        media_types: &["application/vnd.orange.indata"],
        signatures: &[],
        related_formats: &[],
    },
};
