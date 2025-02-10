use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3938229820: FileType = FileType {
    file_format: &FileFormat {
        id: 3_938_229_820,
        source_type: SourceType::Httpd,
        name: "sus calendar",
        extensions: &["sus", "susp"],
        media_types: &["application/vnd.sus-calendar"],
        signatures: &[],
        related_formats: &[],
    },
};
