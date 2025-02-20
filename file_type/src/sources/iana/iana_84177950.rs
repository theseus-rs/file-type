use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_84177950: FileType = FileType {
    file_format: &FileFormat {
        id: 84_177_950,
        source_type: SourceType::Iana,
        name: "vnd.uplanet.cacheop-wbxml",
        extensions: &[],
        media_types: &["application/vnd.uplanet.cacheop-wbxml"],
        signatures: &[],
        related_formats: &[],
    },
};
