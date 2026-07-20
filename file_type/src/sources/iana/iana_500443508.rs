use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_500443508: FileType = FileType {
    file_format: &FileFormat {
        id: 500_443_508,
        source_type: SourceType::Iana,
        name: "vnd.vertifile.pvf",
        extensions: &[],
        media_types: &["application/vnd.vertifile.pvf"],
        signatures: &[],
        related_formats: &[],
    },
};
