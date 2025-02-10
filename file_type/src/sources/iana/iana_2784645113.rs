use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2784645113: FileType = FileType {
    file_format: &FileFormat {
        id: 2_784_645_113,
        source_type: SourceType::Iana,
        name: "vnd.eln+zip",
        extensions: &[],
        media_types: &["application/vnd.eln+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
