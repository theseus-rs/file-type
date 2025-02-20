use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3718926673: FileType = FileType {
    file_format: &FileFormat {
        id: 3_718_926_673,
        source_type: SourceType::Iana,
        name: "vnd.f-secure.mobile",
        extensions: &[],
        media_types: &["application/vnd.f-secure.mobile"],
        signatures: &[],
        related_formats: &[],
    },
};
