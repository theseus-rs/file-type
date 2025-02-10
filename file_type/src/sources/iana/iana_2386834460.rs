use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2386834460: FileType = FileType {
    file_format: &FileFormat {
        id: 2_386_834_460,
        source_type: SourceType::Iana,
        name: "raptorfec",
        extensions: &[],
        media_types: &["video/raptorfec"],
        signatures: &[],
        related_formats: &[],
    },
};
