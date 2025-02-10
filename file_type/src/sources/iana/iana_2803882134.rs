use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2803882134: FileType = FileType {
    file_format: &FileFormat {
        id: 2_803_882_134,
        source_type: SourceType::Iana,
        name: "vnd.hbci",
        extensions: &[],
        media_types: &["application/vnd.hbci"],
        signatures: &[],
        related_formats: &[],
    },
};
