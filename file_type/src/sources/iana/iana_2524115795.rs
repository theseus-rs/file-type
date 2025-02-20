use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2524115795: FileType = FileType {
    file_format: &FileFormat {
        id: 2_524_115_795,
        source_type: SourceType::Iana,
        name: "xfdf",
        extensions: &[],
        media_types: &["application/xfdf"],
        signatures: &[],
        related_formats: &[],
    },
};
