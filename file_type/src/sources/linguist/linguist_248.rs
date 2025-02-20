use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_248: FileType = FileType {
    file_format: &FileFormat {
        id: 248,
        source_type: SourceType::Linguist,
        name: "Nginx",
        extensions: &["nginx", "nginxconf", "vhost"],
        media_types: &["text/x-nginx-conf"],
        signatures: &[],
        related_formats: &[],
    },
};
