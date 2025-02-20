use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2586104738: FileType = FileType {
    file_format: &FileFormat {
        id: 2_586_104_738,
        source_type: SourceType::Httpd,
        name: "dece ttml xml",
        extensions: &["uvt", "uvvt"],
        media_types: &["application/vnd.dece.ttml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
