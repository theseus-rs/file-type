use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3800263245: FileType = FileType {
    file_format: &FileFormat {
        id: 3_800_263_245,
        source_type: SourceType::Httpd,
        name: "msmetafile",
        extensions: &["wmf", "wmz", "emf", "emz"],
        media_types: &["application/x-msmetafile"],
        signatures: &[],
        related_formats: &[],
    },
};
