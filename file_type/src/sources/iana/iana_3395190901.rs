use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3395190901: FileType = FileType {
    file_format: &FileFormat {
        id: 3_395_190_901,
        source_type: SourceType::Iana,
        name: "ISUP",
        extensions: &[],
        media_types: &["application/ISUP"],
        signatures: &[],
        related_formats: &[],
    },
};
