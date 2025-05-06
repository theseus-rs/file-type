use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1024818824: FileType = FileType {
    file_format: &FileFormat {
        id: 1_024_818_824,
        source_type: SourceType::Iana,
        name: "vnd.superfile.super",
        extensions: &[],
        media_types: &["application/vnd.superfile.super"],
        signatures: &[],
        related_formats: &[],
    },
};
