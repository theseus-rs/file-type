use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_159747167: FileType = FileType {
    file_format: &FileFormat {
        id: 159_747_167,
        source_type: SourceType::Httpd,
        name: "3gpp pic bw small",
        extensions: &["psb"],
        media_types: &["application/vnd.3gpp.pic-bw-small"],
        signatures: &[],
        related_formats: &[],
    },
};
