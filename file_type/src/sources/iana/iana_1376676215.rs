use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1376676215: FileType = FileType {
    file_format: &FileFormat {
        id: 1_376_676_215,
        source_type: SourceType::Iana,
        name: "msc-mixer+xml",
        extensions: &[],
        media_types: &["application/msc-mixer+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
