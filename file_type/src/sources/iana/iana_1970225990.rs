use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1970225990: FileType = FileType {
    file_format: &FileFormat {
        id: 1_970_225_990,
        source_type: SourceType::Iana,
        name: "vnd.ms-excel.addin.macroEnabled.12",
        extensions: &[],
        media_types: &["application/vnd.ms-excel.addin.macroEnabled.12"],
        signatures: &[],
        related_formats: &[],
    },
};
