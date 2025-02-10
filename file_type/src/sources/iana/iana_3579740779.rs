use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3579740779: FileType = FileType {
    file_format: &FileFormat {
        id: 3_579_740_779,
        source_type: SourceType::Iana,
        name: "vnd.dir-bi.plate-dl-nosuffix",
        extensions: &[],
        media_types: &["application/vnd.dir-bi.plate-dl-nosuffix"],
        signatures: &[],
        related_formats: &[],
    },
};
