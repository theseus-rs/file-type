use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_484148083: FileType = FileType {
    file_format: &FileFormat {
        id: 484_148_083,
        source_type: SourceType::Httpd,
        name: "muvee style",
        extensions: &["msty"],
        media_types: &["application/vnd.muvee.style"],
        signatures: &[],
        related_formats: &[],
    },
};
