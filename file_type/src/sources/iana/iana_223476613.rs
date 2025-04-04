use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_223476613: FileType = FileType {
    file_format: &FileFormat {
        id: 223_476_613,
        source_type: SourceType::Iana,
        name: "vnd.yellowriver-custom-menu",
        extensions: &[],
        media_types: &["application/vnd.yellowriver-custom-menu"],
        signatures: &[],
        related_formats: &[],
    },
};
