use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2272512056: FileType = FileType {
    file_format: &FileFormat {
        id: 2_272_512_056,
        source_type: SourceType::Httpd,
        name: "hyperstudio",
        extensions: &["stk"],
        media_types: &["application/hyperstudio"],
        signatures: &[],
        related_formats: &[],
    },
};
