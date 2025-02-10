use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2638713880: FileType = FileType {
    file_format: &FileFormat {
        id: 2_638_713_880,
        source_type: SourceType::Iana,
        name: "vnd.sealed.net",
        extensions: &[],
        media_types: &["application/vnd.sealed.net"],
        signatures: &[],
        related_formats: &[],
    },
};
