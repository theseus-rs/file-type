use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3739179471: FileType = FileType {
    file_format: &FileFormat {
        id: 3_739_179_471,
        source_type: SourceType::Iana,
        name: "dskpp+xml",
        extensions: &[],
        media_types: &["application/dskpp+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
