use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2613145387: FileType = FileType {
    file_format: &FileFormat {
        id: 2_613_145_387,
        source_type: SourceType::Iana,
        name: "hej2k",
        extensions: &[],
        media_types: &["image/hej2k"],
        signatures: &[],
        related_formats: &[],
    },
};
