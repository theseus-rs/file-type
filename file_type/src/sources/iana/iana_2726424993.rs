use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2726424993: FileType = FileType {
    file_format: &FileFormat {
        id: 2_726_424_993,
        source_type: SourceType::Iana,
        name: "oebps-package+xml",
        extensions: &[],
        media_types: &["application/oebps-package+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
