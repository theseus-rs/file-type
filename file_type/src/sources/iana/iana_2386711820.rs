use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2386711820: FileType = FileType {
    file_format: &FileFormat {
        id: 2_386_711_820,
        source_type: SourceType::Iana,
        name: "vnd.citationstyles.style+xml",
        extensions: &[],
        media_types: &["application/vnd.citationstyles.style+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
