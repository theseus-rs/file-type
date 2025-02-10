use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2859263887: FileType = FileType {
    file_format: &FileFormat {
        id: 2_859_263_887,
        source_type: SourceType::Httpd,
        name: "lotus approach",
        extensions: &["apr"],
        media_types: &["application/vnd.lotus-approach"],
        signatures: &[],
        related_formats: &[],
    },
};
