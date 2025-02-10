use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3022955634: FileType = FileType {
    file_format: &FileFormat {
        id: 3_022_955_634,
        source_type: SourceType::Httpd,
        name: "novadigm edm",
        extensions: &["edm"],
        media_types: &["application/vnd.novadigm.edm"],
        signatures: &[],
        related_formats: &[],
    },
};
