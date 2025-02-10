use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2707319756: FileType = FileType {
    file_format: &FileFormat {
        id: 2_707_319_756,
        source_type: SourceType::Iana,
        name: "vnd.wap.wmlscriptc",
        extensions: &[],
        media_types: &["application/vnd.wap.wmlscriptc"],
        signatures: &[],
        related_formats: &[],
    },
};
