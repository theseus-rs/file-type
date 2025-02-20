use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2785079285: FileType = FileType {
    file_format: &FileFormat {
        id: 2_785_079_285,
        source_type: SourceType::Iana,
        name: "vnd.immervision-ivp",
        extensions: &[],
        media_types: &["application/vnd.immervision-ivp"],
        signatures: &[],
        related_formats: &[],
    },
};
