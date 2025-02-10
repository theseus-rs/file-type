use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2699209894: FileType = FileType {
    file_format: &FileFormat {
        id: 2_699_209_894,
        source_type: SourceType::Iana,
        name: "vnd.powerbuilder7",
        extensions: &[],
        media_types: &["application/vnd.powerbuilder7"],
        signatures: &[],
        related_formats: &[],
    },
};
