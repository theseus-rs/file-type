use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_676611139: FileType = FileType {
    file_format: &FileFormat {
        id: 676_611_139,
        source_type: SourceType::Httpd,
        name: "vcard",
        extensions: &["vcf"],
        media_types: &["text/x-vcard"],
        signatures: &[],
        related_formats: &[],
    },
};
